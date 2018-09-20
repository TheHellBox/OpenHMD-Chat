local _events = {}

function AddEvent(name, eventname, fn)
  if (_events[ name ] == nil) then
    _events[ name ] = {}
  end
  _events[name][eventname] = fn
end

function CallEvent(name, args)
  if not (_events[ name ] == nil) then
    for k, v in pairs(_events[name]) do
      v(unpack(args))
    end
  end
end
