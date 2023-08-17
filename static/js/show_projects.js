let project_refs = [];

function on_proj_over() {
    for (let i = 1; i < 6; i++) {
        document.getElementById("p" + i.toString()).style.cursor = "pointer";
    }
}

function init_projects() {
    document.getElementById("proj_buttons").style.marginTop = "0.5%";
    document.getElementById("contacts").style.marginTop = "5%";

    document.getElementById("co1").style.display = "none";
    document.getElementById("co2").style.display = "none";
    document.getElementById("co3").style.display = "none";
    document.getElementById("co4").style.display = "none";

    if (document.documentElement.scrollWidth < 1200) {
        document.getElementById("co3").style.display = "block";
    }
    if (document.documentElement.scrollWidth < 768) {
        document.getElementById("co2").style.display = "block";
        document.getElementById("co4").style.display = "block";
        document.getElementById("co3").style.display = "none";
    }
    if (document.documentElement.scrollWidth < 436) {
        document.getElementById("co1").style.display = "block";
        document.getElementById("co2").style.display = "block";
        document.getElementById("co3").style.display = "block";
        document.getElementById("co4").style.display = "block";
    }
    
    document.getElementById("c1").style.height = "40px";
    document.getElementById("c2").style.height = "40px";
    document.getElementById("c1").style.lineHeight = "30px";
    document.getElementById("c2").style.lineHeight = "30px";

    set_projects_refs();
}

function set_projects_refs() {
    project_refs.push("people_vs_undeads", "escape_from_bobrovo", "mematrica", "txt_process", "database");
    for (let i = 1; i < 6; i++) {
        document.getElementById("p" + i.toString()).addEventListener('click', function () {
            window.location.href = project_refs[i - 1]+".html";
        });
    }
}