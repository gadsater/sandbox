package main 

// type Entity struct {
// 	entityType     EntityType
// 	transformStats TransformStats
// 	selected       bool
// }

// const (
// 	PLAYER EntityType = "PLAYER"
// )


type Entity interface {
	GetEntityType() string
	GetProperties() []*Properties
}


type PlayerEntity struct {

}

// type Entity data {}

// let (e Entity) someVal = 3
// let (e Entity) anotherVal = 4

// x := 3
// factorial (x int) -> int :=
//     cond x < 1:
//         send 1
//     else:
//         send x * factorial (x - 1)




// cls mainCls def:
// 	x Int := 0
// 	y Int := 0

// 	@req(mut x Int, mut y Int)
// 	addOneNum := fun (x Int -> Int):
// 		self.x += x
// 		self.y += y



// mut -> self is provided, read and write is allowed
// ref -> self is provided, only read is allowed
// var -> copy is provided, read and write is allowed
// val -> copy is provided, only read is allowed (default)