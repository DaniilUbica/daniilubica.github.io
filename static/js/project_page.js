function init_elements() {
        document.getElementById("proj_header").style.left = "0%";
        document.getElementById("proj_header").style.width = "300px";
        document.getElementById("proj_header").style.fontSize = "30px";

        document.getElementById("description").style.fontSize = "18px";
        document.getElementById("full_description").style.width = "400px";

        document.getElementById("btns").style.paddingTop = "230px";
        
        document.getElementById("try").style.height = "40px";
        document.getElementById("repo").style.height = "40px";
        document.getElementById("try").style.lineHeight = "30px";
        document.getElementById("repo").style.lineHeight = "30px";

        if (document.documentElement.scrollWidth < 436) {
            document.getElementById("description").style.width = "300px";
            document.getElementById("full_description").style.width = "300px";
            document.getElementById("btns").style.paddingTop = "100px";
        }
        if (document.documentElement.scrollWidth < 1480) {
            document.getElementById("btns").style.paddingTop = "250px";
        }
}