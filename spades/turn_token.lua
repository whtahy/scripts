radius = 7.5

-- hand zones
north_hand_id = '4ff856'
south_hand_id = 'ba2279'
east_hand_id = '5ade02'
west_hand_id = '5dcd91'

-- center card zones
north_id = '2bf3ba'
south_id = 'e7c419'
east_id = '5ce4f8'
west_id = '6b1e5c'

function round(x)
    local y = math.floor(math.abs(x) + 0.5)
    if x < 0 then return -y
    else return y
    end
end

function onObjectDestroy(object)
    return true
end

function onObjectLeaveZone(zone, object)
    if object.tag ~= 'Card' then
        return nil
    end

    local x = self.getPosition().x
    local y = self.getPosition().y
    local z = self.getPosition().z

    Wait.condition(
        function()
            if round(x) == 0
                and round(z) == 0
                and #get_cards(zone) == 12 then

                local zone_id = zone.getGUID()
                local new_x, new_z = x, z
                if zone_id == north_hand_id then
                    new_x, new_z = 0, radius
                elseif zone_id == south_hand_id then
                    new_x, new_z = 0, -radius
                elseif zone_id == east_hand_id then
                    new_x, new_z = radius, 0
                elseif zone_id == west_hand_id then
                    new_x, new_z = -radius, 0
                end
                self.setPositionSmooth({new_x, y, new_z})
            end
        end,
        function()
            return onObjectDestroy(object) or object.resting
        end
    )
end

function onObjectEnterZone(zone, object)
    if object.tag ~= 'Card' then
        return nil
    end

    local x = self.getPosition().x
    local y = self.getPosition().y
    local z = self.getPosition().z

    Wait.condition(
        function()
            if is_round_over() then
                local new_x, new_z = rotate(x, z)
                self.setPositionSmooth({new_x, y, new_z})
            end
        end,
        function()
            return onObjectDestroy(object) or object.resting
        end
    )
end

function get_cards(zone)
    local cards = {}
    for _, obj in ipairs(zone.getObjects()) do
        if obj.tag == 'Card' then
            table.insert(cards, obj)
        end
    end
    return cards
end

function is_round_over()
    -- all hands are empty
    for _, p in ipairs(Player.getPlayers()) do
        for _, obj in ipairs(p.getHandObjects()) do
            if obj.tag == 'Card' then
                return false
            end
        end
    end
    -- all card zones are occupied
    return is_occupied(north_id)
        and is_occupied(south_id)
        and is_occupied(east_id)
        and is_occupied(west_id)
end

-- check whether zone contains a card
function is_occupied(zone_guid)
    local cards = get_cards(getObjectFromGUID(zone_guid));
    return #cards == 1 and not cards[1].isSmoothMoving()
end

-- clockwise
function rotate(x, z)
    if round(x) == 0 and math.abs(round(z*10)) == radius*10 then
        return z, x
    elseif math.abs(round(x*10)) == radius*10 and round(z) == 0 then
        return z, -x
    else
        return x, z
    end
end
