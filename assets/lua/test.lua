object_builder = World.create_game_object()
object_builder:with_position(1, 0 ,0)
ent = object_builder:build()
pos = ent:get_position()[1]
print(pos)