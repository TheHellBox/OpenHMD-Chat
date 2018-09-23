ent = World.create_game_object()
ent:with_model("scene_01")
ent:with_position(0, 0, 0)
ent = ent:build()
print(ent:name())


AddEvent("OnClientConnected", "preload_models", function(id)
  script = "World.load_model('./assets/models/scene/scene.obj', 'scene_01')"
  Network.send_lua(script, id)
end)
