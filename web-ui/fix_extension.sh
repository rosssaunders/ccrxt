#!/bin/bash

echo "Starting extension fix process..."

# Ensure dist directory exists
mkdir -p dist/

# Copy extension files
echo "Copying manifest.json and background.js..."
cp manifest.json background.js dist/

# Copy icons directory
echo "Copying icons directory..."
cp -r icons dist/

# Verify files were copied
if [[ ! -f "dist/manifest.json" ]]; then
    echo "ERROR: Failed to copy manifest.json"
    exit 1
fi

if [[ ! -f "dist/background.js" ]]; then
    echo "ERROR: Failed to copy background.js"
    exit 1
fi

if [[ ! -d "dist/icons" ]]; then
    echo "ERROR: Failed to copy icons directory"
    exit 1
fi

# Find the generated JS and WASM files
JS_FILE=$(find dist -name "ccrxt-web-ui*.js" -type f | head -1)
WASM_FILE=$(find dist -name "ccrxt-web-ui*_bg.wasm" -type f | head -1)

if [ -n "$JS_FILE" ] && [ -n "$WASM_FILE" ]; then
    JS_FILENAME=$(basename "$JS_FILE")
    WASM_FILENAME=$(basename "$WASM_FILE")
    
    # Create init.js with correct filenames
    cat > dist/init.js << EOF
import init, * as bindings from './${JS_FILENAME}';

async function initApp() {
    try {
        const wasm = await init('./${WASM_FILENAME}');
        window.wasmBindings = bindings;
        dispatchEvent(new CustomEvent("TrunkApplicationStarted", {detail: {wasm}}));
    } catch (error) {
        console.error('Failed to initialize WASM:', error);
    }
}

initApp();
EOF

    # Fix the HTML file to remove integrity attributes and inline scripts
    echo "Cleaning up HTML file..."
    
    # Remove integrity attributes
    sed -i '' 's/crossorigin="anonymous" integrity="[^"]*"//g' dist/index.html
    sed -i '' 's/crossorigin="anonymous" //g' dist/index.html
    sed -i '' 's/ integrity="[^"]*"//g' dist/index.html
    
    # Remove the entire inline script block and replace with our external script
    # Use a more robust approach to handle multi-line scripts
    if [ -f "dist/index.html" ]; then
        # Create a temporary file to build the new HTML
        awk '
        /<script type="module">/{
            # Found start of inline script, skip everything until </script>
            print "    <script type=\"module\" src=\"./init.js\"></script>"
            in_script = 1
            next
        }
        /<\/script>/ && in_script {
            # Found end of script block, stop skipping
            in_script = 0
            next
        }
        !in_script {
            # Not in script block, print the line
            print
        }
        ' dist/index.html > dist/index.html.tmp && mv dist/index.html.tmp dist/index.html
    fi
    
    # Verify HTML was processed
    if grep -q 'import init' dist/index.html; then
        echo "WARNING: Inline script still present in HTML"
    fi

    echo "Extension files prepared successfully"
else
    echo "Error: Could not find generated JS or WASM files"
    exit 1
fi