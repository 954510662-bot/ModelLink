# ModelLink Visual Assets Generator (Fixed Version)
Add-Type -AssemblyName System.Drawing

$assetsDir = "d:\WSL-Windows.Projects\ModelLink\assets"

function Create-SolidBrush {
    param($color)

    return New-Object System.Drawing.SolidBrush($color)
}

function Draw-LogoIcon {
    param($graphics, $size, $offsetX = 0, $offsetY = 0)

    $scale = $size / 64.0
    $centerX = $offsetX + ($size / 2)
    $centerY = $offsetY + ($size / 2)
    $smallRadius = 4 * $scale
    $largeRadius = 8 * $scale
    $strokeWidth = [int](2 * $scale)
    if ($strokeWidth -lt 1) { $strokeWidth = 1 }

    $color1 = [System.Drawing.Color]::FromArgb(255, 102, 126, 234)
    $color2 = [System.Drawing.Color]::FromArgb(255, 118, 75, 162)

    # Create linear gradient brush
    $rect = New-Object System.Drawing.RectangleF($offsetX, $offsetY, $size, $size)
    $brush = New-Object System.Drawing.Drawing2D.LinearGradientBrush(
        $rect,
        $color1,
        $color2,
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

function Export-LogoIconPng {
    param($size, $filename)

    $bitmap = New-Object System.Drawing.Bitmap($size, $size)
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
    $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
    $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graphics.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality

    # Clear with transparent background
    $graphics.Clear([System.Drawing.Color]::Transparent)

    Draw-LogoIcon -graphics $graphics -size $size

    $filepath = Join-Path $assetsDir $filename
    $bitmap.Save($filepath, [System.Drawing.Imaging.ImageFormat]::Png)

    $graphics.Dispose()
    $bitmap.Dispose()

    Write-Host "Created: $filename"
}

function Export-BackgroundImage {
    $width = 1920
    $height = 1080

    $bitmap = New-Object System.Drawing.Bitmap($width, $height)
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)
    $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality

    # Background gradient
    $bgRect = New-Object System.Drawing.RectangleF(0, 0, $width, $height)
    $bgGradient = New-Object System.Drawing.Drawing2D.LinearGradientBrush(
        $bgRect,
        [System.Drawing.Color]::FromArgb(255, 15, 15, 35),
        [System.Drawing.Color]::FromArgb(255, 26, 26, 62),
        [System.Drawing.Drawing2D.LinearGradientMode]::ForwardDiagonal
    )
    $graphics.FillRectangle($bgGradient, $bgRect)

    # Grid
    $gridPen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(25, 102, 126, 234), 1)
    for ($x = 0; $x -lt $width; $x += 60) {
        $graphics.DrawLine($gridPen, $x, 0, $x, $height)
    }
    for ($y = 0; $y -lt $height; $y += 60) {
        $graphics.DrawLine($gridPen, 0, $y, $width, $y)
    }

    # Connection lines
    $lineColor = [System.Drawing.Color]::FromArgb(100, 102, 126, 234)
    $linePen = New-Object System.Drawing.Pen($lineColor, 2)

    $connections = @(
        @(200, 300, 500, 400, 800, 350),
        @(300, 700, 600, 600, 900, 650),
        @(1200, 200, 1400, 350, 1600, 300),
        @(1100, 700, 1300, 550, 1700, 600),
        @(400, 500, 700, 450, 1000, 520),
        @(1000, 400, 1300, 450, 1600, 400)
    )

    foreach ($conn in $connections) {
        $graphics.DrawLine($linePen, $conn[0], $conn[1], $conn[2], $conn[3])
        $graphics.DrawLine($linePen, $conn[2], $conn[3], $conn[4], $conn[5])
    }

    # Nodes with glow
    $nodes = @(
        @(200, 300), @(500, 400), @(800, 350),
        @(300, 700), @(600, 600), @(900, 650),
        @(1200, 200), @(1400, 350), @(1600, 300),
        @(1100, 700), @(1300, 550), @(1700, 600),
        @(400, 500), @(700, 450), @(1000, 520),
        @(1000, 400), @(1300, 450), @(1600, 400)
    )

    $logoBrush = New-Object System.Drawing.Drawing2D.LinearGradientBrush(
        $bgRect,
        [System.Drawing.Color]::FromArgb(255, 102, 126, 234),
        [System.Drawing.Color]::FromArgb(255, 118, 75, 162),
        [System.Drawing.Drawing2D.LinearGradientMode]::ForwardDiagonal
    )

    foreach ($node in $nodes) {
        $x = $node[0]
        $y = $node[1]

        # Glow effect using multiple semi-transparent ellipses
        for ($i = 5; $i -ge 1; $i--) {
            $alpha = [int](25 * $i)
            $glowBrush = New-Object System.Drawing.SolidBrush([System.Drawing.Color]::FromArgb($alpha, 102, 126, 234))
            $radius = $i * 6
            $graphics.FillEllipse($glowBrush, [int]($x - $radius), [int]($y - $radius), [int]($radius * 2), [int]($radius * 2))
            $glowBrush.Dispose()
        }

        # Node
        $graphics.FillEllipse($logoBrush, [int]($x - 8), [int]($y - 8), 16, 16)
    }

    # Watermark logo in center (semi-transparent)
    $logoSize = 192
    $logoX = [int](($width - $logoSize) / 2)
    $logoY = [int](($height - $logoSize) / 2)

    $logoBitmap = New-Object System.Drawing.Bitmap($logoSize, $logoSize)
    $logoGraphics = [System.Drawing.Graphics]::FromImage($logoBitmap)
    $logoGraphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
    $logoGraphics.Clear([System.Drawing.Color]::Transparent)
    Draw-LogoIcon -graphics $logoGraphics -size $logoSize

    # Draw logo with low opacity using a color matrix
    $colorMatrix = New-Object System.Drawing.Imaging.ColorMatrix
    $colorMatrix.Matrix33 = 0.15  # 15% opacity
    $imageAttributes = New-Object System.Drawing.Imaging.ImageAttributes
    $imageAttributes.SetColorMatrix($colorMatrix)

    $destRect = New-Object System.Drawing.Rectangle($logoX, $logoY, $logoSize, $logoSize)
    $graphics.DrawImage($logoBitmap, $destRect, 0, 0, $logoSize, $logoSize, [System.Drawing.GraphicsUnit]::Pixel, $imageAttributes)

    # Save
    $filepath = Join-Path $assetsDir "modelink-background-1920x1080.png"
    $bitmap.Save($filepath, [System.Drawing.Imaging.ImageFormat]::Png)

    # Cleanup
    $graphics.Dispose()
    $bitmap.Dispose()
    $bgGradient.Dispose()
    $gridPen.Dispose()
    $linePen.Dispose()
    $logoBrush.Dispose()
    $logoGraphics.Dispose()
    $logoBitmap.Dispose()
    $colorMatrix.Dispose()
    $imageAttributes.Dispose()

    Write-Host "Created: modelink-background-1920x1080.png"
}

# Main execution
Write-Host "======================================"
Write-Host "ModelLink Visual Assets Generator"
Write-Host "======================================"
Write-Host ""

Write-Host "Generating Logo Icons (PNG)..."
$sizes = @(512, 256, 128, 64, 32, 16)
foreach ($size in $sizes) {
    Export-LogoIconPng -size $size -filename "modelink-logo-icon-$size.png"
}

Write-Host ""
Write-Host "Generating Background Image..."
Export-BackgroundImage

Write-Host ""
Write-Host "======================================"
Write-Host "All visual assets generated successfully!"
Write-Host "Location: $assetsDir"
Write-Host "======================================"
