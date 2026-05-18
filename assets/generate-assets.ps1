# ModelLink Visual Assets Generator
# This script generates PNG versions of the logo and background images
# Requires: System.Drawing.Common NuGet package

Add-Type -AssemblyName System.Drawing

$assetsDir = "d:\WSL-Windows.Projects\ModelLink\assets"

function Create-GradientBrush {
    param($width, $height)

    $bitmap = New-Object System.Drawing.Bitmap($width, $height)
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)

    $rect = New-Object System.Drawing.Rectangle(0, 0, $width, $height)
    $brush = New-Object System.Drawing.Drawing2D.LinearGradientBrush(
        $rect,
        [System.Drawing.Color]::FromArgb(255, 102, 126, 234),
        [System.Drawing.Color]::FromArgb(255, 118, 75, 162),
        [System.Drawing.Drawing2D.LinearGradientMode]::ForwardDiagonal
    )

    return @{
        Bitmap = $bitmap
        Graphics = $graphics
        Brush = $brush
    }
}

function Draw-LogoIcon {
    param($graphics, $size, $offsetX = 0, $offsetY = 0)

    $scale = $size / 64.0
    $centerX = $offsetX + ($size / 2)
    $centerY = $offsetY + ($size / 2)
    $smallRadius = 4 * $scale
    $largeRadius = 8 * $scale
    $strokeWidth = 2 * $scale

    # Create gradient pen
    $pen = New-Object System.Drawing.Pen($brush, $strokeWidth)

    # Draw cardinal connections
    $graphics.DrawLine($pen, $centerX, ($centerY - $largeRadius * 0.5), $centerX, ($offsetY + $smallRadius))
    $graphics.DrawLine($pen, $centerX, ($centerY + $largeRadius * 0.5), $centerX, ($offsetY + $size - $smallRadius))
    $graphics.DrawLine($pen, ($centerX - $largeRadius * 0.5), $centerY, ($offsetX + $smallRadius), $centerY)
    $graphics.DrawLine($pen, ($centerX + $largeRadius * 0.5), $centerY, ($offsetX + $size - $smallRadius), $centerY)

    # Draw diagonal connections
    $thinPen = New-Object System.Drawing.Pen($brush, (1.5 * $scale))
    $thinPen.Color = [System.Drawing.Color]::FromArgb(180, 102, 126, 234)

    $diagOffset = $smallRadius * 0.6
    $graphics.DrawLine($thinPen, ($centerX - $largeRadius * 0.5), ($centerY - $largeRadius * 0.5), ($offsetX + $smallRadius + $diagOffset), ($offsetY + $smallRadius + $diagOffset))
    $graphics.DrawLine($thinPen, ($centerX + $largeRadius * 0.5), ($centerY - $largeRadius * 0.5), ($offsetX + $size - $smallRadius - $diagOffset), ($offsetY + $smallRadius + $diagOffset))
    $graphics.DrawLine($thinPen, ($centerX - $largeRadius * 0.5), ($centerY + $largeRadius * 0.5), ($offsetX + $smallRadius + $diagOffset), ($offsetY + $size - $smallRadius - $diagOffset))
    $graphics.DrawLine($thinPen, ($centerX + $largeRadius * 0.5), ($centerY + $largeRadius * 0.5), ($offsetX + $size - $smallRadius - $diagOffset), ($offsetY + $size - $smallRadius - $diagOffset))

    # Draw cardinal nodes
    $graphics.FillEllipse($brush, ($centerX - $smallRadius), ($offsetY), ($smallRadius * 2), ($smallRadius * 2))
    $graphics.FillEllipse($brush, ($centerX - $smallRadius), ($offsetY + $size - $smallRadius * 2), ($smallRadius * 2), ($smallRadius * 2))
    $graphics.FillEllipse($brush, ($offsetX), ($centerY - $smallRadius), ($smallRadius * 2), ($smallRadius * 2))
    $graphics.FillEllipse($brush, ($offsetX + $size - $smallRadius * 2), ($centerY - $smallRadius), ($smallRadius * 2), ($smallRadius * 2))

    # Draw diagonal nodes
    $tinyRadius = 3 * $scale
    $graphics.FillEllipse($thinPen.Brush, ($offsetX + $smallRadius + $diagOffset - $tinyRadius), ($offsetY + $smallRadius + $diagOffset - $tinyRadius), ($tinyRadius * 2), ($tinyRadius * 2))
    $graphics.FillEllipse($thinPen.Brush, ($offsetX + $size - $smallRadius - $diagOffset - $tinyRadius), ($offsetY + $smallRadius + $diagOffset - $tinyRadius), ($tinyRadius * 2), ($tinyRadius * 2))
    $graphics.FillEllipse($thinPen.Brush, ($offsetX + $smallRadius + $diagOffset - $tinyRadius), ($offsetY + $size - $smallRadius - $diagOffset - $tinyRadius), ($tinyRadius * 2), ($tinyRadius * 2))
    $graphics.FillEllipse($thinPen.Brush, ($offsetX + $size - $smallRadius - $diagOffset - $tinyRadius), ($offsetY + $size - $smallRadius - $diagOffset - $tinyRadius), ($tinyRadius * 2), ($tinyRadius * 2))

    # Draw central node
    $graphics.FillEllipse($brush, ($centerX - $largeRadius), ($centerY - $largeRadius), ($largeRadius * 2), ($largeRadius * 2))

    $pen.Dispose()
    $thinPen.Dispose()
}

function Export-LogoIconPng {
    param($size, $filename)

    $result = Create-GradientBrush $size $size
    $graphics = $result.Graphics
    $brush = $result.Brush

    $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
    $graphics.InterpolationMode = [System.Drawing.Drawing2D.InterpolationMode]::HighQualityBicubic
    $graphics.PixelOffsetMode = [System.Drawing.Drawing2D.PixelOffsetMode]::HighQuality

    Draw-LogoIcon -graphics $graphics -size $size

    $filepath = Join-Path $assetsDir $filename
    $result.Bitmap.Save($filepath, [System.Drawing.Imaging.ImageFormat]::Png)

    $graphics.Dispose()
    $result.Bitmap.Dispose()
    $brush.Dispose()

    Write-Host "Created: $filename"
}

function Export-BackgroundImage {
    $width = 1920
    $height = 1080

    $bitmap = New-Object System.Drawing.Bitmap($width, $height)
    $graphics = [System.Drawing.Graphics]::FromImage($bitmap)

    $graphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality

    # Background gradient
    $bgRect = New-Object System.Drawing.Rectangle(0, 0, $width, $height)
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

    # Connection lines gradient
    $linePen = New-Object System.Drawing.Pen([System.Drawing.Color]::FromArgb(100, 102, 126, 234), 2)

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

    foreach ($node in $nodes) {
        $x = $node[0]
        $y = $node[1]

        # Glow
        $glowBrush = New-Object System.Drawing.Drawing2D.RadialGradientBrush(
            (New-Object System.Drawing.PointF($x, $y)),
            30,
            (New-Object System.Drawing.PointF($x, $y)),
            0
        )
        $glowBrush.CenterColor = [System.Drawing.Color]::FromArgb(128, 102, 126, 234)
        $glowBrush.SurroundColors = @([System.Drawing.Color]::FromArgb(0, 102, 126, 234))

        $graphics.FillEllipse($glowBrush, ($x - 30), ($y - 30), 60, 60)

        # Node
        $graphics.FillEllipse($bgGradient, ($x - 8), ($y - 8), 16, 16)
    }

    # Watermark logo in center
    $logoSize = 192
    $logoX = ($width - $logoSize) / 2
    $logoY = ($height - $logoSize) / 2

    $logoResult = Create-GradientBrush $logoSize $logoSize
    $logoGraphics = $logoResult.Graphics
    $logoGraphics.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::HighQuality
    Draw-LogoIcon -graphics $logoGraphics -size $logoSize

    $logoBitmap = $logoResult.Bitmap
    $watermarkBrush = New-Object System.Drawing.TextureBrush($logoBitmap)
    $watermarkBrush.TranslateTransform($logoX, $logoY)
    $watermarkBrush.Opacity = 0.15

    $graphics.FillRectangle($watermarkBrush, 0, 0, $width, $height)

    # Save
    $filepath = Join-Path $assetsDir "modelink-background-1920x1080.png"
    $bitmap.Save($filepath, [System.Drawing.Imaging.ImageFormat]::Png)

    # Cleanup
    $graphics.Dispose()
    $bitmap.Dispose()
    $bgGradient.Dispose()
    $gridPen.Dispose()
    $linePen.Dispose()
    $logoGraphics.Dispose()
    $logoBitmap.Dispose()
    $logoResult.Brush.Dispose()

    Write-Host "Created: modelink-background-1920x1080.png"
}

# Main execution
Write-Host "ModelLink Visual Assets Generator"
Write-Host "===================================="
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
Write-Host "All visual assets generated successfully!"
Write-Host "Location: $assetsDir"
