let project_refs = [];

function on_proj_over() {
    for (let i = 1; i < 6; i++) {
        document.getElementById("p" + i.toString()).style.cursor = "pointer";
    }
}

function init_projects() {
    let start_x = 17;
    for (let i = 2; i < 6; i++) {
        document.getElementById("p" + i.toString()).style.left = start_x.toString() + '%';
        start_x += 5;
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