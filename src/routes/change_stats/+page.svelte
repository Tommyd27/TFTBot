
<script>
    // @ts-nocheck 
      
    import { invoke } from "@tauri-apps/api/tauri"
    async function fetch_champs() {
        champs_list = await invoke("retrieve_all_units")
    }
    async function fetch_items() {
        items_list = await invoke("retrieve_all_items")
    }
    let champs_list = []
    let items_list = []
    fetch_champs()
    fetch_items()
    let opacity_champ_error = 0
    let opacity_item_error = 0
    let selected_unit = fetch_champs()
    let selected_item = fetch_items()

    async function handle_submit_update_champ (e) {
        if (check_valid_champ(selected_unit)) {
            await invoke("update_unit", {selectedUnit : selected_unit})
            fetch_champs()
            opacity_champ_error = 0
        }
        else {
            opacity_champ_error = 100
        }
        
    }

    function check_valid_champ(champ) {
        if (champ.ad < 0 || champ.ad > 9999) { return false }
        if (champ.hp <= 0 || champ.health > 9999) { return false }
        if (champ.attack_speed < 0 || champ.attack_speed > 20) { return false }
        if (champ.ar <= 0 || champ.ar > 9999) { return false }
        if (champ.mr <= 0 || champ.mr > 9999) { return false }
        if (champ.mc <= 0 || champ.mc > 9999) { return false }
        if (champ.ra <= 0 || champ.ra > 20) { return false }
        if (champ.sm <= 0 || champ.sm > 9999) { return false }  
        return true
    }

    function check_valid_item(item) {
        if (item.ad < 0 || item.ad > 9999) { return false }
        if (item.ap < 0 || item.ap > 9999) { return false }
        if (item.health < 0 || item.health > 9999) { return false }
        if (item.ar < 0 || item.ar > 9999) { return false }
        if (item.mr < 0 || item.mr > 9999) { return false }
        if (item.attack_speed_modifier < 0 || item.attack_speed_modifier > 20) { return false }
        if (item.cm < 0 || item.cm > 9999) { return false }
        if (item.cr < 0 || item.cr > 100) { return false }
        if (item.ra < 0 || item.ra > 20) { return false }
        if (item.dc < 0 || item.dc > 100) { return false }
        if (item.omnivamp < 0 || item.omnivamp > 9999) { return false}
        if (item.crit_damage < 0 || item.crit_damage > 9999) { return false }
        return true
    }

    async function handle_submit_update_item (e) {
        if (check_valid_item(selected_item)) {
            await invoke("update_item", {selectedItem : selected_item})
            fetch_items()
            opacity_item_error = 0
        }
        else {
            opacity_item_error = 100
        }
        
    }

</script>
<div class="row">
    <div class="column">
        <h1>Change Unit Stats</h1>
        <form on:submit|preventDefault={handle_submit_update_champ}>
            <label>ID</label>
            <select bind:value = {selected_unit} required>
                {#each champs_list as champ}
                    <option value = {champ}>
                        {champ.id}
                    </option>
                {/each}
            </select>
            <br>
            <label>AD</label>
            <input type= "number" bind:value="{selected_unit.ad}" step = 0.01 required><br>
            <label>HP</label>
            <input type="number" bind:value="{selected_unit.hp}" step = 0.01 required><br>
            <label>Attack Speed</label>
            <input type="number" bind:value="{selected_unit.attack_speed}" step = 0.01 required><br>
            <label>Ar</label>
            <input type="number" bind:value="{selected_unit.ar}" step = 0.01 required><br>
            <label>Mr</label>
            <input type="number" bind:value="{selected_unit.mr}" step = 0.01 required><br>
            <label>Mc</label>
            <input type="number" bind:value="{selected_unit.mc}" required><br>
            <label>Ra</label>
            <input type="number" bind:value="{selected_unit.ra}" required><br>
            <label>Sm</label>
            <input type="number" bind:value="{selected_unit.sm}" required><br>

            <button type = "submit">Submit</button>

            <h1 style="opacity : {opacity_champ_error}">Invalid values for some or all variables, please try again.</h1>
        </form>
    </div>
    <div class="column">
        <h1>Change Item Stats</h1>
        <form on:submit|preventDefault={handle_submit_update_item}>
            <label>ID</label>
            <select bind:value = {selected_item} required>
                {#each items_list as item}
                    <option value = {item}>
                        {item.id}
                    </option>
                {/each}
            </select>
            <br>
            <label>AD</label>
            <input type= "number" bind:value="{selected_item.ad}" step = 0.01 required><br>

            <label>AP</label>
            <input type= "number" bind:value="{selected_item.ap}" step = 0.01 required><br>

            <label>Ar</label>
            <input type= "number" bind:value="{selected_item.ar}" step = 0.01 required><br>

            <label>Attack_speed_modifier</label>
            <input type= "number" bind:value="{selected_item.attack_speed_modifier}" step = 0.01 required><br>

            <label>CM</label>
            <input type= "number" bind:value="{selected_item.cm}" required><br>

            <label>Cr</label>
            <input type= "number" bind:value="{selected_item.cr}" required><br>

            <label>crit_damage</label>
            <input type= "number" bind:value="{selected_item.crit_damage}" step = 0.01 required><br>

            <label>Dc</label>
            <input type= "number" bind:value="{selected_item.dc}" required><br>

            <label>Health</label>
            <input type= "number" bind:value="{selected_item.health}" step = 0.01 required><br>

            <label>mr</label>
            <input type= "number" bind:value="{selected_item.mr}" step = 0.01 required><br>

            <label>omnivamp</label>
            <input type= "number" bind:value="{selected_item.omnivamp}" step = 0.01 required><br>

            <label>Range</label>
            <input type= "number" bind:value="{selected_item.ra}" step = 0.01 required><br>

            <button type = "submit">Submit</button>

            <h1 style="opacity : {opacity_item_error}">Invalid values for some or all variables, please try again.</h1>
        </form>
    </div>
</div> 

<style>
    .row {
        display: flex;
    }
    
    .column {
        flex: 50%;
    }
</style>
