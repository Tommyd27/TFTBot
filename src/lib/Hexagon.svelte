<script>
    //@ts-nocheck
    export let champion;
    let color = "aquamarine"
    $: if (champion && champion.team) {
        color = "#ffa895"
    }
    else {
        color = "#49ff2d"
    }
</script>
<div class = "hex">
    
    <div class = "top"></div>

    <!--<div class = "middle"></div>-->
    <button on:click on:mouseup style = "background-color: {color}">
        {#if champion}
            {champion.placed_id}
        {/if}
    </button>   
    
    <div class = "bottom"></div>
    
</div>
<!---->


<style>
    .hex {
        float: left;
        margin-left: 3px;
        margin-bottom: -26px;
        
    }
    .top {
        width: 0;
        border-bottom: 30px solid #6C6;
        border-left: 52px solid transparent;
        border-right: 52px solid transparent;
        z-index: 2;
    }
    .middle {
        width: 104px;
        height: 60px;
        background: #6C6;
        z-index: 2;
    }
    button {
        width: 104px;
        height: 60px;
        background: #6C6;
        z-index: 2;
    }
    button:hover {
        color: black;
    }
    .bottom {
        width: 0;
        border-top: 30px solid #6C6;
        border-left: 52px solid transparent;
        border-right: 52px solid transparent;
        z-index: 2;
    }
    
    h1 {
        z-index: 3;
    }

</style>
