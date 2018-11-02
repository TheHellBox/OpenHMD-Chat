ent = World.create_game_object()
ent:with_model("scene_01")
ent:with_position(0, 0, 0)
ent = ent:build()
print(ent:name())


AddEvent("OnClientConnected", "preload_models", function(id)
script = "Network.DownloadFile('https://46.img.avito.st/640x480/4923397646.jpg', 'myfiles/hey/test/') ";
  Network.SendLua(script, id)
end)
