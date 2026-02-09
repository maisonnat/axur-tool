param(
    [string]$Path = ".",
    [ValidateSet("Xml", "Audit", "Summary")]
    [string]$Mode = "Xml"
)

# Configuration: Templates
$Templates = @{
    "Xml"     = ".agent/templates/xml_packet.hbs"
    "Audit"   = ".agent/templates/audit.hbs"
    "Summary" = ".agent/templates/summary.hbs"
}

# Configuration: Global Exclusion Filters (Noise Reduction)
# These are passed to code2prompt to keep context clean
$GlobalExcludes = @(
    "*.lock",
    "package-lock.json",
    "yarn.lock",
    "Cargo.lock",
    "assets/*",
    "dist/*",
    "target/*",
    "node_modules/*",
    ".git/*",
    "*.png",
    "*.jpg",
    "*.ico",
    "*.svg",
    "*.pdf",
    "*.pptx"
)

# Construct exclude arguments
$ExcludeArgs = $GlobalExcludes | ForEach-Object { "--exclude", "$_" }

# Select Template
$TemplatePath = $Templates[$Mode]

Write-Host "[Context] Ingesting '$Path' in mode '$Mode'..."
Write-Host "[Context] Using template: $TemplatePath"

# Execute code2prompt
# Note: We use splatting for arguments to handle the array correclty
& code2prompt "$Path" --template "$TemplatePath" @ExcludeArgs
