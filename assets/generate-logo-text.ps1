# ModelLink Logo with Text Generator
Add-Type -AssemblyName System.Drawing

$assetsDir = "d:\WSL-Windows.Projects\ModelLink\assets"

function Draw-LogoIcon {
    param($graphics, $size, $offsetX = 0, $offsetY = 0)

    $scale = $size / 64.0
    $centerX = $offsetX + ($size / 2)
    $centerY = $offsetY + ($size / 2)
    $smallRadius = 4 * $scale
    $largeRadius = 8 * $scale
    $strokeWidth = [int](2 * $scale)
    if ($strokeWidth -lt 1) { $strokeWidth = 1 }

    # Create linear gradient brush
    $rect = New-Object System.Drawing.RectangleF($offsetX, $offsetY, $size, $size)
    $brush = New-Object System.Drawing.Drawing2D.LinearGradientBrush(
        $rect,
        [System.Drawing.Color]::FromArgb(255, 102, 126, 234),
        [System.Drawing.Color]::FromArgb(255, 118, 75, 162),
        [System.Drawing.Drawing2D.LinearGradientMode]::ForwardDiagonal
    )

    # Draw cardinal connections
    $pen = New-Object System.Drawing.Pen($brush, $strokeWidth)
    $graphics.DrawLine($pen, [int]$centerX, [int]($centerY - $largeRadius * 0.5), [int]$centerX, [int]($offsetY + $smallRadius))
    $graphics.DrawLine($pen, [int]$centerX, [int]($centerY + $largeRadius * 0.5), [int]$centerX, [int]($offsetY + $size - $smallRadius))
    $graphics.DrawLine($pen, [int]($centerX - $largeRadius * 0.5), [int]$centerY, [int]($offsetX + $smallRadius), [int]$centerY)
    $graphics.DrawLine($pen, [int]($centerX + $largeRadius * 0.5), [int]$centerY, [int]($offsetX + $size - $smallRadius), [int]$centerY)

    # Draw diagonal connections
    $thinWidth = [int](1.5 * $scale)
    if ($thinWidth -lt 1) { $thinWidth = 1 }
    $thinPen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(180, 102, 126, 234), $thinWidth)

    $diagOffset = $smallRadius * 0.6
    $graphics.DrawLine($thinPen, [int]($centerX - $largeRadius * 0.5), [int]($centerY - $largeRadius * 0.5), [int]($offsetX + $smallRadius + $diagOffset), [int]($offsetY + $smallRadius + $diagOffset))
    $graphics.DrawLine($thinPen, [int]($centerX + $largeRadius * 0.5), [int]($centerY - $largeRadius * 0.5), [int]($offsetX + $size - $smallRadius - $diagOffset), [int]($offsetY + $smallRadius + $diagOffset))
    $graphics.DrawLine($thinPen, [int]($centerX - $largeRadius * 0.5), [int]($centerY + $largeRadius * 0.5), [int]($offsetX + $smallRadius + $diagOffset), [int]($offsetY + $size - $smallRadius - $diagOffset))
    $graphics.DrawLine($thinPen, [int]($centerX + $largeRadius * 0.5), [int]($centerY + $largeRadius * 0.5), [int]($offsetX + $size - $smallRadius - $diagOffset), [int]($offsetY + $size - $smallRadius - $diagOffset))

    # Draw cardinal nodes
    $graphics.FillEllipse($brush, [int]($centerX - $smallRadius), [int]($offsetY), [int]($smallRadius * 2), [int]($smallRadius * 2))
    $graphics.FillEllipse($brush, [int]($centerX - $smallRadius), [int]($offsetY + $size - $smallRadius * 2), [int]($smallRadius * 2), [int]($smallRadius * 2))
    $graphics.FillEllipse($brush, [int]($offsetX), [int]($centerY - $smallRadius), [int]($smallRadius * 2), [int]($smallRadius * 2))
    $graphics.FillEllipse($brush, [int]($offsetX + $size - $smallRadius * 2), [int]($centerY - $smallRadius), [int]($smallRadius * 2), [int]($smallRadius * 2))

    # Draw diagonal nodes
    $tinyRadius = 3 * $scale
    $smallBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb(180, 102, 126, 234))
    $graphics.FillEllipse($smallBrush, [int]($offsetX + $smallRadius + $diagOffset - $tinyRadius), [int]($offsetY + $smallRadius + $diagOffset - $tinyRadius), [int]($tinyRadius * 2), [int]($tinyRadius * 2))
    $graphics.FillEllipse($smallBrush, [int]($offsetX + $size - $smallRadius - $diagOffset - $tinyRadius), [int]($offsetY + $smallRadius + $diagOffset - $tinyRadius), [int]($tinyRadius * 2), [int]($tinyRadius * 2))
    $graphics.FillEllipse($smallBrush, [int]($offsetX + $smallRadius + $diagOffset - $tinyRadius), [int]($offsetY + $size - $smallRadius - $diagOffset - $tinyRadius), [int]($tinyRadius * 2), [int]($tinyRadius * 2))
    $graphics.FillEllipse($smallBrush, [int]($offsetX + $size - $smallRadius - $diagOffset - $tinyRadius), [int]($offsetY + $size - $smallRadius - $diagOffset - $tinyRadius), [int]($tinyRadius * 2), [int]($tinyRadius * 2))

    # Draw central node
    $graphics.FillEllipse($brush, [int]($centerX - $largeRadius), [int]($centerY - $largeRadius), [int]($largeRadius * 2), [int]($largeRadius * 2))

    $pen.Dispose()
    $thinPen.Dispose()
    $brush.Dispose()
    $smallBrush.Dispose()
}

function Export-LogoWithTextPng {
    param($width, $height, $filename)

    $bitmap = New-Object System.Drawing.Bitmap($width, $height)
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
    $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
    $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graphics.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit

    # Transparent background
    $graphics.Clear([System.Drawing.Color]::Transparent)

    # Draw logo icon
    $iconSize = [int]($height * 0.875)  # Icon takes 7/8 of height
    Draw-LogoIcon -graphics $graphics -size $iconSize

    # Draw text
    $textX = $iconSize + ($width * 0.05)
    $textY = $height * 0.15

    # Create gradient brush for text
    $textRect = New-Object System.Drawing.RectangleF(0, 0, $width, $height)
    $textBrush = New-Object System.Drawing.Drawing2D.LinearGradientBrush(
        $textRect,
        [System.Drawing.Color]::FromArgb(255, 102, 126, 234),
        [System.Drawing.Color]::FromArgb(255, 118, 75, 162),
        [System.Drawing.Drawing2D.LinearGradientMode]::ForwardDiagonal
    )

    $fontSize = $height * 0.45
    $font = New-Object System.Drawing.Font("Segoe UI", $fontSize, [System.Drawing.FontStyle]::Bold)

    $format = New-Object System.Drawing.StringFormat
    $format.Alignment = [System.Drawing.StringAlignment]::Near
    $format.LineAlignment = [System.Drawing.StringAlignment]::Center

    $textRect2 = New-Object System.Drawing.RectangleF($textX, 0, ($width - $textX - 10), $height)
    $graphics.DrawString("ModelLink", $font, $textBrush, $textRect2, $format)

    # Save
    $filepath = Join-Path $assetsDir $filename
    $bitmap.Save($filepath, [System.Drawing.Imaging.ImageFormat]::Png)

    # Cleanup
    $graphics.Dispose()
    $bitmap.Dispose()
    $textBrush.Dispose()
    $font.Dispose()
    $format.Dispose()

    Write-Host "Created: $filename"
}

# Main
Write-Host "Generating Logo + Text PNG..."
Export-LogoWithTextPng -width 400 -height 128 -filename "modelink-logo-full.png"
Export-LogoWithTextPng -width 300 -height 96 -filename "modelink-logo-full-300x96.png"
Export-LogoWithTextPng -width 200 -height 64 -filename "modelink-logo-full-200x64.png"
Write-Host "Done!"
