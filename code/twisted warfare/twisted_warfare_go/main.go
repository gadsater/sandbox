package main

import (
	"image/color"
	"log"
	"math"
	"slices"
	"time"

	"github.com/hajimehoshi/ebiten/v2"
	"github.com/hajimehoshi/ebiten/v2/text"
	"github.com/hajimehoshi/ebiten/v2/vector"
	"golang.org/x/image/font/basicfont"
)

type EntityType string


const (
	screenWidth  = 1366
	screenHeight = 768
)

type Position struct {
	x, y float32
}

type Ripple struct {
	x, y      float32
	radius    float32
	maxRadius float32
	startTime time.Time
	active    bool
}

type TransformStats struct {
	speed   float32
	size    float32
	currPos Position
	destPos Position
	moving  bool
}

type Button struct {
	x, y, width, height float32
	text                string
}

type Game struct {
	entities                   []*Entity
	ripples                    []Ripple
	spawnButton                Button
	prevLeftMouseButtonPressed bool
}

func NewPlayerEntity(initPos Position) *Entity {
	defaultSpeed := float32(4)
	defaultSize := float32(25)

	return &Entity{entityType: PLAYER, transformStats: TransformStats{speed: defaultSpeed, size: defaultSize, currPos: initPos, destPos: initPos, moving: false}}
}

func (entity *Entity) UpdateEntityPosOnKeyboardInput() {
	if !entity.selected {
		return
	}

	upKeyPressed := ebiten.IsKeyPressed(ebiten.KeyUp) || ebiten.IsKeyPressed(ebiten.KeyW)
	ltKeyPressed := ebiten.IsKeyPressed(ebiten.KeyArrowLeft) || ebiten.IsKeyPressed(ebiten.KeyA)
	dnKeyPressed := ebiten.IsKeyPressed(ebiten.KeyArrowDown) || ebiten.IsKeyPressed(ebiten.KeyS)
	rtKeyPressed := ebiten.IsKeyPressed(ebiten.KeyArrowRight) || ebiten.IsKeyPressed(ebiten.KeyD)

	currPos := &entity.transformStats.currPos
	speed := entity.transformStats.speed

	if upKeyPressed {
		if ltKeyPressed || rtKeyPressed {
			currPos.y -= speed / 2
		} else {
			currPos.y -= speed
		}
	}
	if dnKeyPressed {
		if ltKeyPressed || rtKeyPressed {
			currPos.y += speed / 2
		} else {
			currPos.y += speed
		}
	}
	if ltKeyPressed {
		if upKeyPressed || dnKeyPressed {
			currPos.x -= speed / 2
		} else {
			currPos.x -= speed
		}
	}
	if rtKeyPressed {
		if upKeyPressed || dnKeyPressed {
			currPos.x += speed / 2
		} else {
			currPos.x += speed
		}
	}
}

func (entity *Entity) UpdateEntityPosOnMouseInput() {
	currPos := &entity.transformStats.currPos
	destPos := &entity.transformStats.destPos
	moving := &entity.transformStats.moving
	speed := float64(entity.transformStats.speed)

	if entity.selected && ebiten.IsMouseButtonPressed(ebiten.MouseButtonRight) {
		mouseX, mouseY := ebiten.CursorPosition()
		destPos.x = float32(mouseX)
		destPos.y = float32(mouseY)
		*moving = true
	}

	if *moving {
		dx := float64(destPos.x - currPos.x)
		dy := float64(destPos.y - currPos.y)
		dist := math.Sqrt(dx*dx + dy*dy)

		if dist < speed {
			currPos.x = destPos.x
			currPos.y = destPos.y
			*moving = false
		} else {
			angle := math.Atan2(dy, dx)
			currPos.x += float32(speed * math.Cos(angle))
			currPos.y += float32(speed * math.Sin(angle))
		}
	}
}

func UpdateRippleOnMousePress(ripples *[]Ripple) {
	if ebiten.IsMouseButtonPressed(ebiten.MouseButtonRight) {
		mouseX, mouseY := ebiten.CursorPosition()

		ripple := Ripple{
			x:         float32(mouseX),
			y:         float32(mouseY),
			radius:    0,
			maxRadius: 25,
			startTime: time.Now(),
			active:    true,
		}
		*ripples = append(*ripples, ripple)
	}

	removeInd := []int{}
	rippleSlice := *ripples
	for i := range rippleSlice {
		if rippleSlice[i].active {
			elapsed := time.Since(rippleSlice[i].startTime).Seconds()
			rippleSlice[i].radius = float32(elapsed) * 300
			if rippleSlice[i].radius > rippleSlice[i].maxRadius {
				rippleSlice[i].active = false
				removeInd = append(removeInd, i)
			}
		}
	}

	for i := range removeInd {
		*ripples = slices.Delete(*ripples, i, i+1)
	}
}

func (entity Entity) IsMouseSelectionOnEntity(mouseX, mouseY int) bool {
	currXPos := entity.transformStats.currPos.x
	currYPos := entity.transformStats.currPos.y
	size := entity.transformStats.size

	if (currXPos-size/2 < float32(mouseX)) && (float32(mouseX) < currXPos+size/2) && (currYPos-size/2 < float32(mouseY)) && (float32(mouseY) < currYPos+size/2) {
		return true
	}

	return false
}

func (g *Game) Update() error {
	for i := range g.entities {
		g.entities[i].UpdateEntityPosOnKeyboardInput()
		g.entities[i].UpdateEntityPosOnMouseInput()
	}

	currLeftMouseButtonPressed := ebiten.IsMouseButtonPressed(ebiten.MouseButtonLeft)
	isMouseButtonReleased := !currLeftMouseButtonPressed && g.prevLeftMouseButtonPressed
	g.prevLeftMouseButtonPressed = currLeftMouseButtonPressed

	if isMouseButtonReleased {
		mouseX, mouseY := ebiten.CursorPosition()

		// Check for button click
		if float32(mouseX) >= g.spawnButton.x && float32(mouseX) <= g.spawnButton.x+g.spawnButton.width &&
			float32(mouseY) >= g.spawnButton.y && float32(mouseY) <= g.spawnButton.y+g.spawnButton.height {
			// Spawn new player at center
			newPlayer := NewPlayerEntity(Position{x: screenWidth / 2, y: screenHeight / 2})
			g.entities = append(g.entities, newPlayer)
		}

		for i := range g.entities {
			g.entities[i].selected = false
		}

		// Then select only the topmost entity that matches the mouse position
		for i := len(g.entities) - 1; i >= 0; i-- {
			entity := g.entities[i]
			if entity.IsMouseSelectionOnEntity(mouseX, mouseY) {
				entity.selected = true
				break
			}
		}
	}

	UpdateRippleOnMousePress(&g.ripples)

	return nil
}

func (g *Game) Draw(screen *ebiten.Image) {
	vector.StrokeLine(screen, screenWidth/2, 0, screenWidth/2, screenHeight, 1, color.White, true)
	vector.StrokeLine(screen, 0, screenHeight/2, screenWidth, screenHeight/2, 1, color.White, true)

	for i := range g.entities {
		currPos := g.entities[i].transformStats.currPos
		size := g.entities[i].transformStats.size

		entityColor := color.RGBA{R: 255, G: 255, B: 255, A: 128}

		if g.entities[i].selected {
			entityColor = color.RGBA{R: 255, G: 0, B: 0, A: 128}
		}

		vector.DrawFilledRect(screen, currPos.x-size/2, currPos.y-size/2, size, size, entityColor, true)
	}

	for _, ripple := range g.ripples {
		if ripple.active {
			vector.StrokeCircle(screen, ripple.x, ripple.y, ripple.radius, 1, color.White, true)
		}
	}

	// Draw spawn button
	buttonColor := color.RGBA{R: 0, G: 128, B: 255, A: 255}
	vector.DrawFilledRect(screen, g.spawnButton.x, g.spawnButton.y, g.spawnButton.width, g.spawnButton.height, buttonColor, true)
	vector.StrokeRect(screen, g.spawnButton.x, g.spawnButton.y, g.spawnButton.width, g.spawnButton.height, 2, color.White, true)

	// Draw button text
	textWidth := len(g.spawnButton.text) * 7 // 7 pixels per character
	textHeight := 13                         // font height is 13 pixels
	textX := int(g.spawnButton.x+g.spawnButton.width/2) - textWidth/2
	textY := int(g.spawnButton.y+g.spawnButton.height/2) + textHeight/4

	text.Draw(screen, g.spawnButton.text, basicfont.Face7x13, textX, textY, color.White)
}

func (g *Game) Layout(outsideWidth, outsideHeight int) (int, int) {
	return screenWidth, screenHeight
}

func main() {
	initPos := Position{x: screenWidth / 2, y: screenHeight / 2}
	playerEntity := NewPlayerEntity(initPos)

	game := &Game{
		entities: []*Entity{
			playerEntity,
		},
		spawnButton: Button{
			x:      10,
			y:      float32(screenHeight - 30),
			width:  50,
			height: 20,
			text:   "Spawn",
		},
	}

	ebiten.SetWindowSize(screenWidth, screenHeight)
	ebiten.SetWindowTitle("Twisted Warfare")
	if err := ebiten.RunGame(game); err != nil {
		log.Fatal(err)
	}
}
