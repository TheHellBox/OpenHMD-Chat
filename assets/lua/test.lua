function test_names()
	ent = World.create_game_object()
	ent = ent:build()
	print(ent:name())
end

for i=1, 10 do
	test_names()
end

print("Starting test 2")

objects = World.get_all_objects()
for k, v in ipairs(objects) do
	print(v:name())
end