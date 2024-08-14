extends Node2D

# TODO keep this synchronized with Square.gd
enum SquareState {WHITE, BLACK, ARROW, EMPTY}

@onready var board_squares = $BoardSquares
@onready var black_cli = $BlackCli
@onready var white_cli = $WhiteCli
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
	if black_cli.start_black():
		black_player_type = PlayerType.CLI
	if white_cli.start_white():
		white_player_type = PlayerType.CLI
	_new_game()
	check_for_cli_move()

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

var piece: Array[int] = []
var move: Array[int] = []
func handle_input(x: int, y: int, state: SquareState):
	if piece == []:
		# Selecting a piece
		if (whites_turn and state == SquareState.WHITE) or ((not whites_turn) and state == SquareState.BLACK):
			piece = [x, y]
			return
		else:
			reset_move_state()
	elif move == []:
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
			if whites_turn:
				black_cli.notify_of_move(piece, move, Array([x,y], TYPE_INT, &"", null))
			else:
				white_cli.notify_of_move(piece, move, Array([x,y], TYPE_INT, &"", null))
			whites_turn = not whites_turn
			piece = []
			move = []
			check_for_cli_move()
		else:
			reset_move_state()

func check_for_cli_move():
	if whites_turn and white_player_type == PlayerType.CLI:
		var packed_move = white_cli.get_move()
		var piece = packed_move[0]
		var move = packed_move[1]
		var arrow = packed_move[2]
		squares[piece[0]][piece[1]].mark_empty()
		squares[move[0]][move[1]].mark_white()
		squares[arrow[0]][arrow[1]].mark_arrow()
	elif (not whites_turn) and black_player_type == PlayerType.CLI:
		var packed_move = black_cli.get_move()
		var piece = packed_move[0]
		var move = packed_move[1]
		var arrow = packed_move[2]
		squares[piece[0]][piece[1]].mark_empty()
		squares[move[0]][move[1]].mark_black()
		squares[arrow[0]][arrow[1]].mark_arrow()
	else:
		return
	whites_turn = not whites_turn
	check_for_cli_move()

func _on_cancel():
	reset_move_state()

func reset_move_state():
	if move != []:
		# Reset the temporary move
		squares[move[0]][move[1]].mark_empty()
		if whites_turn:
			squares[piece[0]][piece[1]].mark_white()
		else:
			squares[piece[0]][piece[1]].mark_black()
	piece = []
	move = []
