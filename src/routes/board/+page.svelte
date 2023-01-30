<script>
    // @ts-nocheck 
    import UnitItemDragDrop from "$lib/UnitItemDragDrop.svelte";
    import Hexagon from "$lib/Hexagon.svelte";
    import HexagonIndent from "$lib/HexagonIndent.svelte";
    import { invoke } from "@tauri-apps/api/tauri"


    let ids = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z']
    let time_unit = 10
    let time_till_draw = 10000

    async function fetch_champs() {
        champs_list = await invoke("retrieve_all_units")
        selected_champ = champs_list[0]
    }
    async function fetch_items() {
        items_list = await invoke("retrieve_all_items")
    }

    function increase_star_level() {
        grid[show_pos[0]][show_pos[1]].star_level += 1
        if (grid[show_pos[0]][show_pos[1]].star_level > 3) {
            grid[show_pos[0]][show_pos[1]].star_level = 1
        }
        update_show(show_pos[0], show_pos[1])
    }

    function change_team() {
        grid[show_pos[0]][show_pos[1]].team = !grid[show_pos[0]][show_pos[1]].team
    }
    function hex_click(i, j) {
        champ = grid[i][j]
        show_pos = [i, j]
        if (champ == show) {
            show = null
        }
        else {
            show = champ
            pos = [80 + 130 * j, 200 + 80 * i];
        }
    }
    function create_grid() {
        let grid = new Array(8)
        for(var i = 0; i < grid.length; i++) {
            grid[i] = new Array(8)
        }
        return grid
    }

    function on_mouse_down(champ, e) {
        selected_champ = structuredClone(champ)
        id = champ.id
        show_drag = 100
        left = e.clientX + 5
        top = e.clientY + 5
    }

    function on_mouse_up(i, j) {
        if (show_drag == 100) {
            show_drag = 0
            if (selected_champ.attack_speed_modifier) { 
                if (grid[i][j]) {
                    if (grid[i][j].items[0] == 0) {
                        grid[i][j].items[0] = selected_champ.id
                    }
                    else if (grid[i][j].items[1] == 0) {
                        grid[i][j].items[1] = selected_champ.id
                    }
                    else if (grid[i][j].items[2] == 0) {
                        grid[i][j].items[2] = selected_champ.id
                    }
                    else {
                        grid[i][j].items[0] = selected_champ.id
                    }
                }
                update_show(i, j)
            }
            else {
                selected_champ.items = new Array(0, 0, 0)
                selected_champ.star_level = 1
                selected_champ.placed_id = generate_id()
                selected_champ.team = false;
                grid[i][j] = selected_champ
            }
        }
        
        
    }

    function on_mouse_move(e) {
        if (show_drag == 100) {
            left += e.movementX;
			top += e.movementY;
		}
    }

    function delete_click() {
        grid[show_pos[0]][show_pos[1]] = null;
        show = null;
    }

    function update_show(i, j) {
        if (show_pos[0] == i && show_pos[1] == j && show) {
            show = grid[i][j]
        }
    }

    function log_base_n(base, number) {
        return Math.floor(Math.log(number) / Math.log(base));
    }
    function generate_id() {
        num_placed_champs += 1
        if(num_placed_champs == 0) {
            return "A"
        }
        let num_to_id = num_placed_champs
        let id_str = ""
        let cap = log_base_n(ids.length, num_to_id)
        for(let i = 0; i < cap + 1; i++) {
            if(num_to_id == 0){
                id_str += "A"
                break
            }
            let log_n = log_base_n(ids.length, num_to_id)
            let divisor = Math.pow(ids.length, log_n)
            let id_index = Math.floor(num_to_id / divisor)
            id_str += ids[id_index]
            num_to_id -= divisor * id_index
        }
        return id_str
    }
    let show_pos = [0, 0]
    let id = 0
    let grid = create_grid()
    //grid[1][2] = obj
    let champs_list = []
    let items_list = []
    let show;
    let num_placed_champs = -1
    let show_drag = 0;
    let pos = [0, 0];
    let selected_champ;
    let left = 0;
    let top = 0;


    fetch_champs()
    fetch_items()

    class PlacedChamp {
        constructor(id, items, star_level, location) {
            this.id = id
            this.items = items
            this.star = star_level
            this.location = location
        }
    }

    class Location {
        constructor(x, y) {
            this.x = x
            this.y = y
        }
    }

    async function handle_submit() {
        let player_one_champs = []
        let player_two_champs = []
        for (let i = 0; i < grid.length; i++) {
            for (let j = 0; j < grid[i].length; j++) {
                if (grid[i][j]) {
                    let placed_champ = new PlacedChamp(grid[i][j].id, grid[i][j].items, grid[i][j].star_level, new Location(i, j))
                    if (grid[i][j].team) {
                        player_one_champs.push(placed_champ)
                    }
                    else {
                        player_two_champs.push(placed_champ)
                    }
                }
            }
        }
        await invoke("submit_board", {"playerOneChamps": player_one_champs, "playerTwoChamps": player_two_champs, "timeUnit": time_unit, "timeTillDraw" : time_till_draw})
        window.location.href = "/board/battle"
    }

    let champ = champs_list[0]
</script>
<svelte:window on:mousemove={on_mouse_move} />
<h1>Board</h1>

<div class="row">
    <div class="column1_noselect">
        <h2 style = "font-size: 20px">Champs</h2><br>
        {#each champs_list as champ}
            <UnitItemDragDrop champ_or_item = {champ} on:mousedown = {(e) => on_mouse_down(champ, e)}></UnitItemDragDrop>
        {/each}
        <h2 style = "font-size: 20px">Items</h2><br>
        {#each items_list as item}
            <UnitItemDragDrop champ_or_item = {item} on:mousedown = {(e) => on_mouse_down(item, e)}></UnitItemDragDrop>
        {/each}
    </div>
    <div class="column2">
        {#each grid as row, i}
            {#if i % 2 == 1}
                <HexagonIndent></HexagonIndent>
            {/if}
            {#each row as hex, j}
                <Hexagon champion = {grid[i][j]} on:click = {() => hex_click(i, j)} on:mouseup = {() => on_mouse_up(i, j)}></Hexagon>
            {/each}
            <div class = "hex-row"></div>
        {/each}
    </div>
    <div class="column3">
        <form on:submit={handle_submit}>
            <button type = "submit">Start Battle</button><br>
            <label>Time Unit</label>
            <input type = "number" min = 1 max = 1000 bind:value = {time_unit} required><br>
            <label>Ticks Till Draw</label>
            <input type = "number" min = 100 max = 100000 bind:value = {time_till_draw} required>
        </form>
    </div>
</div>

<div class = "drag_container" style = "position: absolute; left: {left}px; top: {top}px; opacity: {show_drag}">
    <h1>{id}</h1>
</div>

{#if show} 
    <!--<div style = "position: absolute; top: {pos[0]}; right: {pos[1]}" class = "info_bar">-->
    <div style = "position: absolute; top: {pos[1]}px; left: {pos[0]}px" class = "info_bar">
        <h1>Champ: {show.placed_id}</h1>
        <h1>Type: {show.id}</h1>
        <h1>Star Level: {show.star_level}</h1>
        <h1>Items: {show.items}</h1>
        <button on:click = {delete_click}>Delete Unit</button>
        <button on:click = {increase_star_level}>Increase Star Level</button>
        <button on:click = {change_team}>Change Team</button>
    </div>
{/if}


<style>
    .row {
        display: flex;
    }
    
    .column1_noselect {
        flex: 5%;
        background-color: grey;
        display:grid;
        grid-template-columns: 40px 40px;
        user-select: none;
    }

    .column3 {
        flex: 15%;
        background-color: grey;
        display:grid;
    }

    .column2 {
        flex : 80%;
        background-color: aliceblue;
    }
    .hex-row {
        clear: left;
    }

    .info_bar {
        width: 140px;
        height: 160px;
        font-size: 10px;
        background-color: aliceblue;
        z-index: 3;
    }

    .drag_container {
        border: solid 1px gray;
        background-color: aliceblue;
    }
</style>

