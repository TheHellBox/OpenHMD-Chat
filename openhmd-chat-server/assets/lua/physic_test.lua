physic_test = World.CreateGameObject()
physic_test:with_model("cube")
physic_test:with_position(0, 0, 0)
physic_test:with_scale(5, 0.1, 5)
physic_test = physic_test:build()

collider = World.CreateRigidBody()
collider:WithSize(5, 0.1, 5)
collider = collider:Build()

physic_test:AttachRigidBody(collider)

cube = World.CreateGameObject()
cube:with_model("cube")
cube = cube:build()

ray = World.CreateRayCast()
ray:WithPosition(0, 9, 0)
ray:WithDirection(0, -1, 0)
result = ray:Build():Point()

cube:SetPosition(result[1], result[2], result[3])


for k=0,100 do
    cube = World.CreateGameObject()
    cube:with_model("wooden_box")
    cube:with_position(0, k / 5, 0)
    cube = cube:build()

    rigidbody = World.CreateRigidBody()
    rigidbody:Static(false)
    rigidbody = rigidbody:Build()

    cube:AttachRigidBody(rigidbody)
end
