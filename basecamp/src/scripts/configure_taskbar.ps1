# Remove Task View
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name ShowTaskViewButton -Value 0 -Type DWord -Force
# Remove Widgets
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name TaskbarDa -Value 0 -Type DWord -Force
# Remove Co-Pilot
New-Item -Path "HKCU:\Software\Policies\Microsoft\Windows" -Name WindowsCopilot
Set-ItemProperty -Path "HKCU:\Software\Policies\Microsoft\Windows\WindowsCopilot" -Name TurnOffWindowsCopilot -Value 1 -Type DWord -Force
# Align Centre
Set-ItemProperty -Path "HKCU:\Software\Microsoft\Windows\CurrentVersion\Explorer\Advanced" -Name TaskbarAl -Value 0 -Type DWord -Force
# Set Search Icon To Hidden
Set-ItemProperty -Path "HKCU:\SOFTWARE\Microsoft\Windows\CurrentVersion\Search" -Name SearchboxTaskbarMode -Value 0 -Force