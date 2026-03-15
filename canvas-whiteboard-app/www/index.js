// Bootstrap: load the wasm module produced by wasm-pack.
import init, { App, CanvasController } from '../pkg/canvas_whiteboard.js';

async function main() {
    await init();

    const app = new App("My Vault");
    const ctrl = new CanvasController();

    const canvas = document.getElementById("whiteboard");
    const ctx = canvas.getContext("2d");

    // ---- Resize canvas to fill parent ----
    function resize() {
        const rect = canvas.parentElement.getBoundingClientRect();
        canvas.width = rect.width;
        canvas.height = rect.height - document.getElementById("toolbar").offsetHeight;
    }
    window.addEventListener("resize", resize);
    resize();

    // ---- State ----
    let activeShelfId = null;

    // ---- Sidebar ----
    const shelfTree = document.getElementById("shelf-tree");
    const btnNewShelf = document.getElementById("btn-new-shelf");

    function refreshSidebar() {
        const vault = JSON.parse(app.vault_json());
        document.getElementById("vault-name").textContent = vault.name;
        shelfTree.innerHTML = "";
        for (const shelf of vault.shelves) {
            const el = document.createElement("div");
            el.className = "shelf-item" + (shelf.id === activeShelfId ? " active" : "");
            el.textContent = shelf.name;
            el.addEventListener("click", () => {
                activeShelfId = shelf.id;
                refreshSidebar();
                repaint();
            });
            shelfTree.appendChild(el);
        }
    }

    btnNewShelf.addEventListener("click", () => {
        const name = prompt("Shelf name:");
        if (name) {
            const id = app.create_shelf(name);
            activeShelfId = id;
            refreshSidebar();
        }
    });

    // ---- Toolbar ----
    const btnSelect = document.getElementById("btn-select");
    const btnDraw = document.getElementById("btn-draw");
    const btnText = document.getElementById("btn-text");
    const penColor = document.getElementById("pen-color");
    const penWidth = document.getElementById("pen-width");
    const zoomDisplay = document.getElementById("zoom-display");

    let mode = "select"; // "select" | "draw" | "text"

    function setMode(m) {
        mode = m;
        btnSelect.classList.toggle("active", m === "select");
        btnDraw.classList.toggle("active", m === "draw");
        btnText.classList.toggle("active", m === "text");
        canvas.style.cursor = m === "draw" ? "crosshair" : m === "text" ? "text" : "default";
    }

    btnSelect.addEventListener("click", () => setMode("select"));
    btnDraw.addEventListener("click", () => setMode("draw"));
    btnText.addEventListener("click", () => setMode("text"));
    penColor.addEventListener("input", () => ctrl.set_pen_color(penColor.value));
    penWidth.addEventListener("input", () => ctrl.set_pen_width(parseFloat(penWidth.value)));

    // ---- Canvas interaction ----
    let isPanning = false;
    let lastX = 0, lastY = 0;

    canvas.addEventListener("mousedown", (e) => {
        if (!activeShelfId) return;
        if (e.button === 1 || (e.button === 0 && e.shiftKey)) {
            // Middle-click or shift+click → pan
            isPanning = true;
            lastX = e.clientX;
            lastY = e.clientY;
            return;
        }
        if (mode === "text" && e.button === 0) {
            const coords = ctrl.screen_to_world(e.offsetX, e.offsetY);
            const blockId = app.create_block(activeShelfId, coords[0], coords[1]);
            if (blockId) {
                setMode("select");
                repaint();
            }
        }
    });

    canvas.addEventListener("mousemove", (e) => {
        if (isPanning) {
            const dx = e.clientX - lastX;
            const dy = e.clientY - lastY;
            ctrl.pan(dx, dy);
            lastX = e.clientX;
            lastY = e.clientY;
            repaint();
        }
    });

    canvas.addEventListener("mouseup", () => {
        isPanning = false;
    });

    canvas.addEventListener("wheel", (e) => {
        e.preventDefault();
        const factor = e.deltaY < 0 ? 1.1 : 0.9;
        ctrl.zoom(factor, e.offsetX, e.offsetY);
        zoomDisplay.textContent = Math.round(ctrl.zoom_level() * 100) + "%";
        repaint();
    }, { passive: false });

    // ---- Text editing ----
    const textOverlay = document.getElementById("text-overlay");
    const blockEditor = document.getElementById("block-editor");
    const mdPreview = document.getElementById("markdown-preview");
    let editingBlockId = null;

    canvas.addEventListener("dblclick", (e) => {
        if (!activeShelfId) return;
        // For simplicity in the scaffold: open editor at click position.
        const coords = ctrl.screen_to_world(e.offsetX, e.offsetY);
        // Try to find a block at this position (simplified: use vault JSON).
        const vault = JSON.parse(app.vault_json());
        const shelf = vault.shelves.find(s => s.id === activeShelfId);
        if (!shelf) return;
        for (const block of shelf.blocks) {
            const w = block.width || 200;
            const h = block.height || 100;
            if (coords[0] >= block.x && coords[0] <= block.x + w &&
                coords[1] >= block.y && coords[1] <= block.y + h) {
                editingBlockId = block.id;
                textOverlay.classList.remove("hidden");
                textOverlay.style.left = e.clientX + "px";
                textOverlay.style.top = e.clientY + "px";
                blockEditor.value = block.content.Text || "";
                blockEditor.focus();
                return;
            }
        }
    });

    blockEditor.addEventListener("blur", () => {
        if (editingBlockId && activeShelfId) {
            app.set_block_text(activeShelfId, editingBlockId, blockEditor.value);
            // Show markdown preview.
            const html = app.render_block_markdown(activeShelfId, editingBlockId);
            if (html) {
                mdPreview.innerHTML = html;
                mdPreview.style.left = textOverlay.style.left;
                mdPreview.style.top = textOverlay.style.top;
                mdPreview.classList.remove("hidden");
                setTimeout(() => mdPreview.classList.add("hidden"), 3000);
            }
            editingBlockId = null;
            textOverlay.classList.add("hidden");
            repaint();
        }
    });

    // ---- Script console (toggle with backtick) ----
    const scriptPanel = document.getElementById("script-panel");
    const scriptInput = document.getElementById("script-input");
    const scriptOutput = document.getElementById("script-output");
    const btnRunScript = document.getElementById("btn-run-script");

    document.addEventListener("keydown", (e) => {
        if (e.key === "`" && !e.repeat) {
            scriptPanel.classList.toggle("hidden");
            if (!scriptPanel.classList.contains("hidden")) {
                scriptInput.focus();
            }
            e.preventDefault();
        }
    });

    btnRunScript.addEventListener("click", () => {
        const result = app.eval_script(scriptInput.value);
        scriptOutput.textContent = result;
    });

    // ---- Repaint ----
    function repaint() {
        // Delegate to a simple JS-side painter since we serialise block data.
        const vault = JSON.parse(app.vault_json());
        const shelf = vault.shelves.find(s => s.id === activeShelfId);
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        if (!shelf) return;

        ctx.save();
        ctx.translate(ctrl.offset_x(), ctrl.offset_y());
        ctx.scale(ctrl.zoom_level(), ctrl.zoom_level());

        for (const block of shelf.blocks) {
            const w = block.width || 200;
            const h = block.height || 100;
            // Background
            ctx.fillStyle = "#ffffff";
            ctx.fillRect(block.x, block.y, w, h);
            // Border
            ctx.strokeStyle = "#cccccc";
            ctx.lineWidth = 1;
            ctx.strokeRect(block.x, block.y, w, h);
            // Text
            if (block.content.Text) {
                ctx.fillStyle = "#333333";
                ctx.font = "14px monospace";
                ctx.fillText(block.content.Text, block.x + 8, block.y + 20);
            }
            // Drawing strokes
            if (block.content.Drawing) {
                for (const stroke of block.content.Drawing) {
                    if (stroke.points.length < 2) continue;
                    ctx.beginPath();
                    ctx.strokeStyle = stroke.color;
                    ctx.lineWidth = stroke.width;
                    ctx.moveTo(stroke.points[0].x, stroke.points[0].y);
                    for (let i = 1; i < stroke.points.length; i++) {
                        ctx.lineTo(stroke.points[i].x, stroke.points[i].y);
                    }
                    ctx.stroke();
                }
            }
        }
        ctx.restore();
    }

    // Initial render.
    refreshSidebar();
    repaint();
}

main().catch(console.error);
