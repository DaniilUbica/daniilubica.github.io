let project_refs = [];

function on_proj_over() {
    for (let i = 1; i < 6; i++) {
        document.getElementById("p" + i.toString()).style.cursor = "pointer";
    }
}

function init_projects() {
    let start_x = "22%";
    let t;
    if (document.documentElement.scrollWidth - 8 < 350) {
        t = parseInt(start_x.slice(0, -1));
        t -= (350 - (document.documentElement.scrollWidth - 8)) / 1.5;
    }
    else if (document.documentElement.scrollWidth - 8 < 380) {
        t = parseInt(start_x.slice(0, -1));
        t -= (380 - (document.documentElement.scrollWidth - 8)) / 2;
    }
    else if (document.documentElement.scrollWidth - 8 < 450) {
        t = parseInt(start_x.slice(0, -1));
        t -= (450 - (document.documentElement.scrollWidth - 8)) / 1.1;
    }
    else if (document.documentElement.scrollWidth - 8 < 780) {
        t = parseInt(start_x.slice(0, -1));
        t -= (780 - (document.documentElement.scrollWidth - 8)) / 0.9;
    }
    else if (document.documentElement.scrollWidth - 8 < 1050) {
        t = parseInt(start_x.slice(0, -1));
        t -= (1050 - (document.documentElement.scrollWidth - 8)) / 1.9;
    }
    else if (document.documentElement.scrollWidth - 8 < 2000) {
        t = parseInt(start_x.slice(0, -1));
        t -= (1900 - (document.documentElement.scrollWidth - 8)) / 5;
    }
    else if (document.documentElement.scrollWidth - 8 < 2600) {
        t = parseInt(start_x.slice(0, -1));
        t -= (1900 - (document.documentElement.scrollWidth - 8)) / 83;
    }
    else {
        t = parseInt(start_x.slice(0, -1));
    }

    start_x = t.toString() + '%'

    for (let i = 1; i < 6; i++) {
        let t = parseInt(start_x);
        document.getElementById("p" + i.toString()).style.left = t.toString() + '%';
        //start_x = (parseFloat(start_x.slice(0, -1)) + 2).toString() + '%';
    }
    set_projects_refs();
}

function set_projects_refs() {
    project_refs.push("people_vs_undeads", "escape_from_bobrovo", "mematrica", "txt_process", "database");
    for (let i = 1; i < 6; i++) {
        document.getElementById("p" + i.toString()).addEventListener('click', function () {
            window.location.href = project_refs[i - 1];
        });
    }
}