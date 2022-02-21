deck_zone_id = 'a0f2d7'
reset_button_id = '0e20c2'
debug_button_id = 'e94e7d'

function onLoad()
    for _, obj in ipairs(getAllObjects()) do
        if obj.hasTag('Static') or obj.hasTag('Table') then
            obj.interactable = false
            obj.drag_selectable = false
        end
    end

    deck_zone = getObjectFromGUID(deck_zone_id)

    reset_button = getObjectFromGUID(reset_button_id)
    reset_button.createButton({
        click_function  = 'reset',
        label           = 'Reset',
        position        = {0, 0.5, 0},
        width           = 800,
        height          = 400,
        font_size       = 340,
        color           = {0.5, 0.5, 0.5},
        font_color      = {1, 1, 1}
    })

    debug_button = getObjectFromGUID(debug_button_id)
    debug_button.createButton({
        click_function  = 'debug',
        label           = 'Debug!',
        position        = {0, 0.5, 0},
        width           = 1100,
        height          = 400,
        font_size       = 340,
        color           = {0.5, 0.5, 0.5},
        font_color      = {1, 1, 1}
    })
end

function debug()
end

function round_to_180(x)
    return math.floor(x / 180 + 0.5) * 180
end

function reset()
    local players = Player.getPlayers()
    local exist_player = false

    -- failsafe: check hand size
    for _, p in ipairs(players) do
        if p.getHandCount() == 0 then
            goto continue
        elseif #get_hand(p) > 0 then
            print('Hand is not empty!')
            return nil
        end
        exist_player = true
        ::continue::
    end

    -- failsafe: check hand
    if not exist_player then
        print('No hand zone!')
        return nil
    end

    -- rotate cards face down -> fetch cards into deck
    for _, c in ipairs(get_cards()) do
        c.use_hands = false
        c.setRotationSmooth({
            x = round_to_180(c.getRotation().x),
            y = round_to_180(c.getRotation().y),
            z = 180
        })
        c.setPositionSmooth(deck_zone.getPosition())
        Wait.time(function() c.use_hands = true end, 0.5)
    end

    local shuffle = false -- synchronize

    -- shuffle deck
    Wait.condition(
        function()
            get_deck().shuffle()
            Wait.time(function() shuffle = true end, 0.7)
        end,
        -- wait until deck is ready
        function()
            return get_deck() ~= nil
        end
    )

    local new_hand = false -- synchronize

    -- deal cards
    Wait.condition(
        function()
            deal_hands()
            new_hand = true
        end,
        -- wait until deck is shuffled
        function()
            return shuffle and get_deck() ~= nil
        end
    )

    -- sort cards in hand
    for _, p in ipairs(players) do
        Wait.condition(
            function()
                sort_hand(p)
            end,
            -- wait until hand is ready
            function()
                return new_hand and #get_hand(p) == 13
            end
        )
    end
end

function get_deck()
    for _, obj in ipairs(deck_zone.getObjects()) do
        if obj.tag == 'Deck'
            and obj.getQuantity() == 52
            and obj.resting then
            return obj
        end
    end
end

function get_cards()
    local cards = {}
    for _, obj in ipairs(getAllObjects()) do
        if obj.tag == 'Card' or obj.tag == 'Deck' then
            table.insert(cards, obj)
        end
    end
    return cards
end

function get_hand(player)
    local hand = {}
    for _, obj in ipairs(player.getHandObjects()) do
        if obj.tag == 'Card' and obj.resting then
            table.insert(hand, obj)
        end
    end
    return hand
end

function deal_hands()
    local deck = get_deck()
    for _, p in ipairs(Player.getPlayers()) do
        for i = 1, 13 do
            deck.dealToColorWithOffset({0,0,0}, false, p.color)
        end
    end
end

function sort_hand(player)
    local hand = {}
    local positions = {}

    -- get cards + positions
    for _, c in ipairs(player.getHandObjects()) do
        table.insert(hand, c)
        table.insert(positions, c.getPosition())
    end

    -- get sort order, by card description
    table.sort(
        hand,
        function(a, b)
            return a.getDescription() < b.getDescription()
        end
    )

    -- reposition cards
    for i, c in ipairs(hand) do
        c.setPosition(positions[i])
    end
end
