function init_elements() {
    console.log(document.documentElement.scrollWidth);
    if (document.documentElement.scrollWidth - 8 < 450) {
        document.getElementById("proj_header").style.left = "0%";
        document.getElementById("proj_header").style.width = "200px";
        document.getElementById("proj_header").style.fontSize = "22px";

        document.getElementById("description").style.left = "-5%";
        document.getElementById("description").style.width = "300px";

        document.getElementById("full_description").style.width = "300px";

        document.getElementById("first").style.width = "300px";
        document.getElementById("second").style.width = "300px";
        document.getElementById("first").style.height = "150px";
        document.getElementById("second").style.height = "150px";

        document.getElementById("first").style.top = "70%";
        document.getElementById("second").style.top = "90%";

        document.getElementById("try").style.top = "110%";
        document.getElementById("try").style.left = "0%";
        document.getElementById("repo").style.top = "112%";
        document.getElementById("repo").style.left = "0%";

        document.getElementById("try").style.display = "block";
        document.getElementById("repo").style.display = "block";
    }
    else if (document.documentElement.scrollWidth - 8 < 800) {
        document.getElementById("first").style.top = "55%";
        document.getElementById("second").style.top = "95%";

        document.getElementById("try").style.top = "135%";
        document.getElementById("repo").style.top = "135%";

        document.getElementById("try").style.left = "25%";
        document.getElementById("repo").style.left = "35%";
    }
    else if (document.documentElement.scrollWidth - 8 < 1200) {
        document.getElementById("proj_header").style.left = "5%";
        document.getElementById("description").style.left = "5%";
        document.getElementById("full_description").style.left = "5%";

        document.getElementById("first").style.top = "55%";
        document.getElementById("second").style.top = "95%";

        document.getElementById("try").style.top = "135%";
        document.getElementById("repo").style.top = "135%";

        document.getElementById("try").style.left = "31%";
        document.getElementById("repo").style.left = "41%";
    }
    else if (document.documentElement.scrollWidth - 8 < 1600){
        document.getElementById("first").style.top = "55%";
        document.getElementById("second").style.top = "95%";

        document.getElementById("try").style.top = "135%";
        document.getElementById("repo").style.top = "135%";

        document.getElementById("try").style.left = "33%";
        document.getElementById("repo").style.left = "43%";
    }
    else if (document.documentElement.scrollWidth - 8 < 2000){
        document.getElementById("first").style.top = "55%";
        document.getElementById("second").style.top = "95%";

        document.getElementById("try").style.top = "135%";
        document.getElementById("repo").style.top = "135%";

        document.getElementById("try").style.left = "37%";
        document.getElementById("repo").style.left = "47%";
    }
    else if (document.documentElement.scrollWidth - 8 < 2600){
        document.getElementById("first").style.width = "800px";
        document.getElementById("second").style.width = "800px";
        document.getElementById("first").style.height = "500px";
        document.getElementById("second").style.height = "500px";

        document.getElementById("first").style.top = "55%";
        document.getElementById("second").style.top = "95%";

        document.getElementById("try").style.top = "135%";
        document.getElementById("repo").style.top = "135%";

        document.getElementById("proj_header").style.left = "0%";
        document.getElementById("proj_header").style.width = "600px";
        document.getElementById("proj_header").style.fontSize = "42px";
        document.getElementById("proj_header").style.border = "5px solid";

        document.getElementById("description").style.width = "600px";
        document.getElementById("description").style.fontSize = "24px";

        document.getElementById("full_description").style.width = "700px";
        document.getElementById("full_description").style.fontSize = "22px";

        document.getElementById("try").style.width = "300px";
        document.getElementById("repo").style.width = "300px";

        document.getElementById("try").style.left = "32%";
        document.getElementById("repo").style.left = "42%";
    }
}