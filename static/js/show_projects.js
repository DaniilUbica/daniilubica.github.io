let project_refs = [];

function on_proj_over() {
    for (let i = 1; i < 6; i++) {
        document.getElementById("p" + i.toString()).style.cursor = "pointer";
    }
}

function init_projects() {

    document.getElementById("c1").style.display = "block";
    document.getElementById("c2").style.display = "block";
    document.getElementById("c1").style.top = "58%";
    document.getElementById("c2").style.top = "59%";
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
            window.location.href = project_refs[i - 1];
        });
    }
}