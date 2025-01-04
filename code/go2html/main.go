package main

import (
	"fmt"
	"strings"
)

type ElementType string

const (
	container ElementType = "container"
)

type Property interface{}

type Element struct {
	name ElementType
	// properties []*Property
	children []*Element
}

func (element *Element) Render() string {
	return element.renderWithIndentation(0)
}

func (element *Element) renderWithIndentation(level int) string {
	if element.name == "container" {
		outputString := strings.Repeat(" ", level*2) + string("<"+element.name+">\n")
		for _, elem := range element.children {
			outputString += elem.renderWithIndentation(level+1) + "\n"
		}
		outputString += strings.Repeat(" ", level*2) + string("</"+element.name+">")

		return outputString
	}

	return "default"
}

func Container(children []*Element) *Element {
	return &Element{name: container, children: children}
}

func main() {
	fmt.Println(
		Container(
			[]*Element{
				Container(
					[]*Element{
						Container(nil),
					},
				),
				Container(nil),
			},
		).Render(),
	)

}
