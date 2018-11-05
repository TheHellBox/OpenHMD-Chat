ent = World.CreateGameObject()
ent:with_model("scene_01")
ent:with_position(0, 0, 0)
ent = ent:build()

AddEvent("OnClientConnected", "preload_models", function(id)
    player = Player.GetByID(id)
    player:SendLua([[print("Welcome to server!")]])
end)
