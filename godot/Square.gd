extends Node2D

signal click(x,y)
signal cancel

@onready var black = $Black
@onready var white = $White
@onready var arrow = $Arrow
@onready var background = $Background

# TODO keep this synchronized with Scene.gd
enum State {WHITE, BLACK, ARROW, EMPTY}

var x: int
var y: int
var state: State = State.EMPTY

func _update():
	if (x+y) % 2 == 0:
		background.color = Color(0.9, 0.9, 0.9)
	else:
		background.color = Color(0.4, 0.4, 0.4)
	if state == State.WHITE:
		black.visible = false
		white.visible = true
		arrow.visible = false
	elif state == State.BLACK:
		black.visible = true
		white.visible = false
		arrow.visible = false
	elif state == State.ARROW:
		black.visible = false
		white.visible = false
		arrow.visible = true
	else:
		black.visible = false
		white.visible = false
		arrow.visible = false

func _draw():
	_update()
	#draw_circle(Vector2(50,50), 50, Color(.9, .9, .9))

func _ready():
	_update()

func _input_event(_vp, event, _idx):
	if event is InputEventMouseButton and event.pressed:
		if event.button_mask == 1:
			click.emit(x, y, state)
		elif event.button_mask == 2:
			cancel.emit()

func mark_white():
	state = State.WHITE
	_update()

func mark_black():
	state = State.BLACK
	_update()

func mark_arrow():
	state = State.ARROW
	_update()

func mark_empty():
	state = State.EMPTY
	_update()
