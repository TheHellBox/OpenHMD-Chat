function create_game_object()
	ent = World.create_game_object()
	ent = ent:build()
	return ent
end

for i=1, 10 do
	create_game_object()
end

objects = World.get_all_objects()
for k, v in ipairs(objects) do
	print(v:name())
end
