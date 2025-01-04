package main

import (
	"image"
	"image/color"
	"log"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/ebitenutil"
)

const (
	defaultScreenWidth  = 640
	defaultScreenHeight = 480
)

type Sprite struct {
	img  *ebiten.Image
	x, y float64
}

type Game struct {
	player  *Sprite
	sprites []*Sprite
}

func (g *Game) Update() error {
	if ebiten.IsKeyPressed(ebiten.KeyUp) {
		g.player.y -= 2
	}
	if ebiten.IsKeyPressed(ebiten.KeyLeft) {
		g.player.x -= 2
	}
	if ebiten.IsKeyPressed(ebiten.KeyDown) {
		g.player.y += 2
	}
	if ebiten.IsKeyPressed(ebiten.KeyRight) {
		g.player.x += 2
	}

	for _, sprite := range g.sprites {
		// Follow the player
		if sprite.x < g.player.x {
			sprite.x += 0.5
		} else if sprite.x > g.player.x {
			sprite.x -= 0.5
		}

		if sprite.y < g.player.y {
			sprite.y += 0.5
		} else if sprite.y > g.player.y {
			sprite.y -= 0.5
		}
	}

	return nil
}

func (g *Game) Draw(screen *ebiten.Image) {
	screen.Fill(color.RGBA{120, 180, 255, 255})

	opts := ebiten.DrawImageOptions{}
	opts.GeoM.Translate(g.player.x, g.player.y)

	// draw the player
	screen.DrawImage(
		g.player.img.SubImage(
			image.Rect(0, 0, 16, 16),
		).(*ebiten.Image),
		&opts,
	)
	opts.GeoM.Reset()

	for _, sprite := range g.sprites {
		opts.GeoM.Translate(sprite.x, sprite.y)

		screen.DrawImage(
			sprite.img.SubImage(
				image.Rect(0, 0, 16, 16),
			).(*ebiten.Image),
			&opts,
		)
		opts.GeoM.Reset()
	}
}

func (g *Game) Layout(outsideWidth, outsideHeight int) (screenWidth, screenHeight int) {
	return ebiten.WindowSize()
}

func main() {
	ebiten.SetWindowSize(defaultScreenWidth, defaultScreenHeight)
	ebiten.SetWindowTitle("Hello, World!")
	ebiten.SetWindowResizingMode(ebiten.WindowResizingModeEnabled)

	playerImage, _, err := ebitenutil.NewImageFromFile("assets/images/ninja.png")
	if err != nil {
		log.Fatal(err)
	}

	skeletonImage, _, err := ebitenutil.NewImageFromFile("assets/images/skeleton.png")
	if err != nil {
		log.Fatal(err)
	}

	game := Game{
		player: &Sprite{
			img: playerImage,
			x:   50,
			y:   50,
		},
		sprites: []*Sprite{
			{
				img: skeletonImage,
				x:   75,
				y:   75,
			},
			{
				img: skeletonImage,
				x:   100,
				y:   100,
			},
			{
				img: skeletonImage,
				x:   125,
				y:   125,
			},
		},
	}

	if err := ebiten.RunGame(&game); err != nil {
		log.Fatal(err)
	}
}
