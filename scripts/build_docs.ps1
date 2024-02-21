<#
.SYNOPSIS
Generate documentation for all features in a Cargo project.

.DESCRIPTION
This script generates documentation for all features in a Cargo project using the "cargo doc --all-features" command.

.NOTES
File Name      : generate-doc.ps1
Author         : Your Name
Prerequisite   : Cargo must be installed on the system.

#>
Start-Process cargo -ArgumentList "doc --all-features" -NoNewWindow -Wait
