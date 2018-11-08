physic_test = World.CreateGameObject()
physic_test:with_model("cube")
physic_test:with_position(0, 0, 0)
physic_test:with_scale(5, 0.1, 5)
physic_test = physic_test:build()

collider = World.CreateCollider()
collider:WithSize(5, 0.1, 5)
collider = collider:Build()

physic_test:AttachCollider(collider)


for k=0,10 do
    cube = World.CreateGameObject()
    cube:with_model("cube")
    cube:with_position(0, k / 5, 0)
    cube = cube:build()

    collider = World.CreateCollider()
    collider:Static(false)
    collider = collider:Build()

    cube:AttachCollider(collider)
end
