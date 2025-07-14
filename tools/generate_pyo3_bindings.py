#!/usr/bin/env python3
"""
PyO3 Binding Generator

This script analyzes the Rust codebase and generates PyO3 bindings automatically.
It uses cargo expand to get the expanded code and then analyzes it to generate bindings.
"""

import os
import re
import subprocess
import json
from pathlib import Path
from typing import List, Dict, Any

class PyO3BindingGenerator:
    def __init__(self, venues_path: str, output_path: str):
        self.venues_path = Path(venues_path)
        self.output_path = Path(output_path)
        self.output_path.mkdir(parents=True, exist_ok=True)
        
    def analyze_codebase(self):
        """Analyze the entire venues codebase"""
        print("Analyzing codebase...")
        
        # Find all Rust files
        rust_files = list(self.venues_path.rglob("*.rs"))
        
        bindings = {}
        
        for file in rust_files:
            print(f"Processing {file}")
            bindings.update(self.analyze_file(file))
            
        return bindings
    
    def analyze_file(self, file_path: Path) -> Dict[str, Any]:
        """Analyze a single Rust file"""
        try:
            with open(file_path, 'r') as f:
                content = f.read()
            
            # Extract structs, enums, and impls
            structs = self.extract_structs(content)
            enums = self.extract_enums(content)
            impls = self.extract_impls(content)
            
            return {
                'file': str(file_path),
                'structs': structs,
                'enums': enums,
                'impls': impls
            }
        except Exception as e:
            print(f"Error processing {file_path}: {e}")
            return {}
    
    def extract_structs(self, content: str) -> List[Dict[str, Any]]:
        """Extract struct definitions from Rust code"""
        structs = []
        
        # Regex to match struct definitions
        struct_pattern = r'#\[derive\([^\]]+\)\]\s*pub struct (\w+)\s*\{([^}]+)\}'
        
        for match in re.finditer(struct_pattern, content, re.MULTILINE | re.DOTALL):
            struct_name = match.group(1)
            fields_text = match.group(2)
            
            # Extract fields
            fields = self.extract_fields(fields_text)
            
            # Determine if this should be exposed to Python
            should_expose = self.should_expose_struct(struct_name, fields)
            
            if should_expose:
                structs.append({
                    'name': struct_name,
                    'fields': fields,
                    'type': 'struct'
                })
        
        return structs
    
    def extract_enums(self, content: str) -> List[Dict[str, Any]]:
        """Extract enum definitions from Rust code"""
        enums = []
        
        # Regex to match enum definitions
        enum_pattern = r'#\[derive\([^\]]+\)\]\s*pub enum (\w+)\s*\{([^}]+)\}'
        
        for match in re.finditer(enum_pattern, content, re.MULTILINE | re.DOTALL):
            enum_name = match.group(1)
            variants_text = match.group(2)
            
            # Extract variants
            variants = self.extract_enum_variants(variants_text)
            
            enums.append({
                'name': enum_name,
                'variants': variants,
                'type': 'enum'
            })
        
        return enums
    
    def extract_impls(self, content: str) -> List[Dict[str, Any]]:
        """Extract impl blocks from Rust code"""
        impls = []
        
        # Regex to match impl blocks
        impl_pattern = r'impl\s+(\w+)\s*\{([^}]+)\}'
        
        for match in re.finditer(impl_pattern, content, re.MULTILINE | re.DOTALL):
            impl_name = match.group(1)
            methods_text = match.group(2)
            
            # Extract public methods
            methods = self.extract_methods(methods_text)
            
            if methods and self.should_expose_impl(impl_name):
                impls.append({
                    'name': impl_name,
                    'methods': methods,
                    'type': 'impl'
                })
        
        return impls
    
    def extract_fields(self, fields_text: str) -> List[Dict[str, str]]:
        """Extract field definitions from struct body"""
        fields = []
        
        # Simple field extraction - can be improved with proper parsing
        field_pattern = r'pub\s+(\w+):\s*([^,\n]+)'
        
        for match in re.finditer(field_pattern, fields_text):
            field_name = match.group(1)
            field_type = match.group(2).strip()
            
            fields.append({
                'name': field_name,
                'type': field_type
            })
        
        return fields
    
    def extract_enum_variants(self, variants_text: str) -> List[str]:
        """Extract enum variants"""
        variants = []
        
        # Simple variant extraction
        variant_pattern = r'(\w+)(?:\s*\{[^}]*\})?'
        
        for match in re.finditer(variant_pattern, variants_text):
            variant_name = match.group(1)
            if variant_name and not variant_name.startswith('//'):
                variants.append(variant_name)
        
        return variants
    
    def extract_methods(self, methods_text: str) -> List[Dict[str, Any]]:
        """Extract method definitions from impl block"""
        methods = []
        
        # Regex to match public async methods
        method_pattern = r'pub\s+async\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*([^{]+)'
        
        for match in re.finditer(method_pattern, methods_text):
            method_name = match.group(1)
            return_type = match.group(2).strip()
            
            methods.append({
                'name': method_name,
                'return_type': return_type,
                'is_async': True
            })
        
        # Also match non-async public methods
        method_pattern = r'pub\s+fn\s+(\w+)\s*\([^)]*\)\s*->\s*([^{]+)'
        
        for match in re.finditer(method_pattern, methods_text):
            method_name = match.group(1)
            return_type = match.group(2).strip()
            
            methods.append({
                'name': method_name,
                'return_type': return_type,
                'is_async': False
            })
        
        return methods
    
    def should_expose_struct(self, name: str, fields: List[Dict[str, str]]) -> bool:
        """Determine if a struct should be exposed to Python"""
        expose_patterns = [
            'Request', 'Response', 'Client', 'Error', 'Info', 'Data',
            'Order', 'Trade', 'Account', 'Balance', 'Position', 'Ticker'
        ]
        
        return any(pattern in name for pattern in expose_patterns)
    
    def should_expose_impl(self, name: str) -> bool:
        """Determine if an impl block should be exposed to Python"""
        expose_patterns = ['Client', 'Request']
        return any(pattern in name for pattern in expose_patterns)
    
    def generate_bindings(self, bindings: Dict[str, Any]):
        """Generate PyO3 bindings from analyzed code"""
        print("Generating PyO3 bindings...")
        
        # Group by venue
        venues = {}
        
        for file_data in bindings.values():
            if not file_data:
                continue
                
            file_path = file_data['file']
            venue_name = self.extract_venue_name(file_path)
            
            if venue_name not in venues:
                venues[venue_name] = {
                    'structs': [],
                    'enums': [],
                    'impls': []
                }
            
            venues[venue_name]['structs'].extend(file_data.get('structs', []))
            venues[venue_name]['enums'].extend(file_data.get('enums', []))
            venues[venue_name]['impls'].extend(file_data.get('impls', []))
        
        # Generate bindings for each venue
        for venue_name, venue_data in venues.items():
            self.generate_venue_bindings(venue_name, venue_data)
    
    def extract_venue_name(self, file_path: str) -> str:
        """Extract venue name from file path"""
        path_parts = Path(file_path).parts
        if 'venues' in path_parts and 'src' in path_parts:
            venues_index = path_parts.index('venues')
            src_index = path_parts.index('src')
            if src_index > venues_index and src_index + 1 < len(path_parts):
                return path_parts[src_index + 1]
        return 'unknown'
    
    def generate_venue_bindings(self, venue_name: str, venue_data: Dict[str, Any]):
        """Generate bindings for a specific venue"""
        output_file = self.output_path / f"{venue_name}.rs"
        
        # Generate Rust code
        rust_code = self.generate_rust_bindings(venue_name, venue_data)
        
        with open(output_file, 'w') as f:
            f.write(rust_code)
        
        print(f"Generated bindings for {venue_name}")
    
    def generate_rust_bindings(self, venue_name: str, venue_data: Dict[str, Any]) -> str:
        """Generate Rust code for PyO3 bindings"""
        lines = [
            "use pyo3::prelude::*;",
            "use pyo3_asyncio;",
            f"use venues::{venue_name};",
            "",
        ]
        
        # Generate struct bindings
        for struct_data in venue_data['structs']:
            lines.extend(self.generate_struct_binding(struct_data))
            lines.append("")
        
        # Generate enum bindings
        for enum_data in venue_data['enums']:
            lines.extend(self.generate_enum_binding(enum_data))
            lines.append("")
        
        # Generate impl bindings
        for impl_data in venue_data['impls']:
            lines.extend(self.generate_impl_binding(impl_data))
            lines.append("")
        
        # Generate module function
        lines.extend([
            f"pub fn create_{venue_name}_module(py: Python) -> PyResult<&PyModule> {{",
            f"    let m = PyModule::new(py, \"{venue_name}\")?;",
            "",
        ])
        
        # Add classes to module
        for struct_data in venue_data['structs']:
            lines.append(f"    m.add_class::<{struct_data['name']}>()?;")
        
        for enum_data in venue_data['enums']:
            lines.append(f"    m.add_class::<{enum_data['name']}>()?;")
        
        lines.extend([
            "    Ok(m)",
            "}",
        ])
        
        return '\n'.join(lines)
    
    def generate_struct_binding(self, struct_data: Dict[str, Any]) -> List[str]:
        """Generate PyO3 binding for a struct"""
        name = struct_data['name']
        fields = struct_data['fields']
        
        lines = [
            "#[pyclass]",
            "#[derive(Clone)]",
            f"pub struct {name} {{",
            f"    inner: {name}::{name},",
            "}",
            "",
            "#[pymethods]",
            f"impl {name} {{",
            "    #[new]",
            "    fn new() -> Self {",
            "        Self {",
            f"            inner: {name}::{name}::default(),",
            "        }",
            "    }",
        ]
        
        # Add getters for fields
        for field in fields:
            field_name = field['name']
            field_type = self.convert_rust_type_to_python(field['type'])
            
            lines.extend([
                "",
                "    #[getter]",
                f"    fn {field_name}(&self) -> PyResult<{field_type}> {{",
                f"        Ok(self.inner.{field_name}.clone().into())",
                "    }",
            ])
        
        lines.append("}")
        
        return lines
    
    def generate_enum_binding(self, enum_data: Dict[str, Any]) -> List[str]:
        """Generate PyO3 binding for an enum"""
        name = enum_data['name']
        variants = enum_data['variants']
        
        lines = [
            "#[pyclass]",
            "#[derive(Clone)]",
            f"pub enum {name} {{",
        ]
        
        for variant in variants:
            lines.append(f"    {variant},")
        
        lines.append("}")
        
        return lines
    
    def generate_impl_binding(self, impl_data: Dict[str, Any]) -> List[str]:
        """Generate PyO3 binding for an impl block"""
        name = impl_data['name']
        methods = impl_data['methods']
        
        lines = [
            "#[pymethods]",
            f"impl {name} {{",
        ]
        
        for method in methods:
            method_name = method['name']
            is_async = method['is_async']
            
            if is_async:
                lines.extend([
                    "",
                    f"    fn {method_name}<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {{",
                    "        let client = self.inner.clone();",
                    "        pyo3_asyncio::tokio::future_into_py(py, async move {",
                    f"            client.{method_name}().await",
                    "        })",
                    "    }",
                ])
            else:
                lines.extend([
                    "",
                    f"    fn {method_name}(&self) -> PyResult<()> {{",
                    f"        self.inner.{method_name}();",
                    "        Ok(())",
                    "    }",
                ])
        
        lines.append("}")
        
        return lines
    
    def convert_rust_type_to_python(self, rust_type: str) -> str:
        """Convert Rust type to Python-compatible type"""
        type_mapping = {
            'String': 'String',
            'u64': 'u64',
            'i64': 'i64',
            'f64': 'f64',
            'bool': 'bool',
            'Decimal': 'String',
        }
        
        # Handle Option types
        if rust_type.startswith('Option<') and rust_type.endswith('>'):
            inner_type = rust_type[7:-1]
            return f"Option<{self.convert_rust_type_to_python(inner_type)}>"
        
        # Handle Vec types
        if rust_type.startswith('Vec<') and rust_type.endswith('>'):
            inner_type = rust_type[4:-1]
            return f"Vec<{self.convert_rust_type_to_python(inner_type)}>"
        
        return type_mapping.get(rust_type, 'PyObject')

def main():
    generator = PyO3BindingGenerator(
        venues_path="venues/src",
        output_path="python-bindings/src/generated"
    )
    
    # Analyze the codebase
    bindings = generator.analyze_codebase()
    
    # Generate PyO3 bindings
    generator.generate_bindings(bindings)
    
    print("PyO3 binding generation complete!")

if __name__ == "__main__":
    main()
