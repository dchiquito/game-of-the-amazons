extends Node2D

# TODO keep this synchronized with Square.gd
enum SquareState {WHITE, BLACK, ARROW, EMPTY}

@onready var board_squares = $BoardSquares
var square_scene = preload("res://Square.tscn")
var squares = {}

var whites_turn: bool = true

enum PlayerType {UI, CLI}
var white_player_type: PlayerType = PlayerType.UI
var black_player_type: PlayerType = PlayerType.UI

func _ready():
	for x in range(0, 10):
		squares[x] = {}
		for y in range(0, 10):
			var square = square_scene.instantiate()
			square.x = x
			square.y = y
			square.position = Vector2(x*100, (9-y)*100)
			board_squares.add_child(square)
			squares[x][y] = square
			square.click.connect(_on_click)
			square.cancel.connect(_on_cancel)

	print("ya")
	print(OS.get_cmdline_args())
	_new_game()

func _new_game():
	for x in range(0, 10):
		for y in range(0, 10):
			squares[x][y].mark_empty()
	squares[0][3].mark_white()
	squares[3][0].mark_white()
	squares[6][0].mark_white()
	squares[9][3].mark_white()
	squares[0][6].mark_black()
	squares[3][9].mark_black()
	squares[6][9].mark_black()
	squares[9][6].mark_black()

func _on_click(x: int, y: int, state: SquareState):
	if (whites_turn and white_player_type == PlayerType.UI) or ((not whites_turn) and black_player_type == PlayerType.UI):
		handle_input(x, y, state)

var piece = null
var move = null
func handle_input(x: int, y: int, state: SquareState):
	print("input ",x,y,state,piece,move)
	if piece == null:
		# Selecting a piece
		if (whites_turn and state == SquareState.WHITE) or ((not whites_turn) and state == SquareState.BLACK):
			piece = [x, y]
			return
		else:
			reset_move_state()
	elif move == null:
		# Selecting a move for a piece
		if state == SquareState.EMPTY:
			# TODO check for legal move
			move = [x, y]
			# Move the piece temporarily for visualization
			if whites_turn:
				squares[x][y].mark_white()
			else:
				squares[x][y].mark_black()
			squares[piece[0]][piece[1]].mark_empty()
			return
		else:
			reset_move_state()
	else:
		# Selecting an arrow
		if state == SquareState.EMPTY:
			# TODO check for legal move
			squares[x][y].mark_arrow()
			# TODO emit move to CLI player
			whites_turn = not whites_turn
			piece = null
			move = null
		else:
			reset_move_state()

func _on_cancel():
	reset_move_state()

func reset_move_state():
	if move != null:
		# Reset the temporary move
		squares[move[0]][move[1]].mark_empty()
		if whites_turn:
			squares[piece[0]][piece[1]].mark_white()
		else:
			squares[piece[0]][piece[1]].mark_black()
	piece = null
	move = null
