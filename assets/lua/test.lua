game_object_builder = World.create_game_object()
game_object_builder:with_position(0, 6, 0)
game_object = game_object_builder:build()
print("Test0")

x = 0

while true do
	x = x + 0.000001
	game_object:set_position(0, x, 0)
end
print("Test1")