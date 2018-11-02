btn1 = Ui.AddButton("button_pressed", {"hello", "there"})
btn1:SetSize(300, 300)
btn1:SetPosition(128, 300)
btn1:SetLabel("Say hello")

AddEvent("button_pressed", "print_something", function(arg_1, arg_2)
	print(arg_1 .. " " .. arg_2)
end)
