pub fn generate(project_name: &str) -> String {
    format!(
        r#"# Documentation {name}

## Overview

Documentation générée par AlphaRust.

## Installation

## Usage

## API

## Examples
"#,
        name = project_name
    )
}
