var data=$("#hiddeninputs").serializeArray();
    
var serializeObj={};
$(data).each(function(){
    serializeObj[this.name]=this.value;
});