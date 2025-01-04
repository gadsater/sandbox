package main

type Properties interface {
	GetPropertyType() string
}

type AttackProperties struct {
	attackPower     float32
	powerMultiplier float32
	attackRange     float32
	rangeMultiplier float32
}

func (a *AttackProperties) GetPropertyType() string {
	return "Attack"
}

type HealthProperties struct {
	maxHealth        float32
	currentHealth    float32
	healthMultiplier float32
}

func (h *HealthProperties) GetPropertyType() string {
	return "Health"
}

type DefenseProperties struct {
	defenseArmor    float32
	powerMultiplier float32
}

func (d *DefenseProperties) GetPropertyType() string {
	return "Defense"
}

type MovementProperties struct {
	movementSpeed   float32
	speedMultiplier float32
}

func (m *MovementProperties) GetPropertyType() string {
	return "Movement"
}