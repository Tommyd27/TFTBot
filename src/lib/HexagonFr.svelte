<script>
    //@ts-nocheck
    export let champion;
</script>
<div class="hex">
    <div class="hex inner">
        <div class="hex inner2"></div>
    </div>
</div>

<!---->


<style>
    .hex {
        margin-top: 70px;
        width: 208px;
        height: 120px;
        background: #6C6;
        position: relative;
        z-index: 0;
    }
    .hex:before, .hex:after {
        content:"";
        border-left: 104px solid transparent;
        border-right: 104px solid transparent;
        position: absolute;
    }
    .hex:before {
        top: -59px;
        border-bottom: 60px solid #6C6;
    }
    .hex:after {
        bottom: -59px;
        border-top: 60px solid #6C6;
    }

    .hex.inner {
        background-color:blue;
        -webkit-transform: scale(.9, .9);
        -moz-transform: scale(.9, .9);
        transform: scale(.9, .9);
        z-index:3;
    }
    .hex.inner:before {
        border-bottom: 60px solid blue;
        z-index:1;
    }
    .hex.inner:after {
        border-top: 60px solid blue;
        z-index:1;
    }

</style>
