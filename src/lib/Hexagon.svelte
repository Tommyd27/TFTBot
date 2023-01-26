<div class = "hex">
    <div class = "top"></div>
    <div class = "middle"></div>
    <div class = "bottom"></div>
</div>

<div class = "hex">
    <div class = "top"></div>
    <div class = "middle"></div>
    <div class = "bottom"></div>
</div>
<div class = "hex-row"></div>

<div class = "hex-row even">
    <div class = "top"></div>
    <div class = "middle"></div>
    <div class = "bottom"></div>
</div>


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
    }
    .middle {
        width: 104px;
        height: 60px;
        background: #6C6;
    }
    .bottom {
        width: 0;
        border-top: 30px solid #6C6;
        border-left: 52px solid transparent;
        border-right: 52px solid transparent;
    }
    .hex-row {
        clear: left;
    }
    .hex-row.even {
        margin-left: 56px;
    }

</style>
