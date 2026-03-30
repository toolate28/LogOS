import os

BASE_DIR = r"c:\Users\Matthew Ruhnau\reson8\coherence-mcp\coherence-site\public"

ENDPOINTS = {
    "reforge/index.html": {
        "title": "Reforge OS Dashboard",
        "color": "#6ea8fe",
        "desc": "[ Main Theming Core // TriWeavon Reforge Base ]"
    },
    "lattice/index.html": {
        "title": "Lattice RAG",
        "color": "#8b8b9e",
        "desc": "[ Vector Retrieval // Codebase Subconscious ]"
    },
    "meta-map/index.html": {
        "title": "Meta-Map Topology",
        "color": "#f59e0b",
        "desc": "[ Live TDA Point Cloud // Visual Intelligence ]"
    },
    "terminal/index.html": {
        "title": "Argonath Terminal",
        "color": "#ff5555",
        "desc": "[ Sovereign CLI Access // Invariant Alpha ]"
    },
    "inspector/index.html": {
        "title": "AST Inspector",
        "color": "#d946ef",
        "desc": "[ Deep Semantic Tooltip // Node Validation ]"
    }
}

TEMPLATE = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title}</title>
    <link href="https://fonts.googleapis.com/css2?family=Outfit:wght@300;400;600;800&family=JetBrains+Mono:wght@400;700&display=swap" rel="stylesheet">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>
    <style>
        body {{ margin:0; padding:0; background:#020205; font-family:'Outfit', sans-serif; overflow:hidden; color:#fff; }}
        #bg-canvas {{ position:absolute; top:0; left:0; width:100vw; height:100vh; z-index:0; opacity:0.3; }}
        
        .glass-layer {{
            position: absolute; top: 0; left: 0; width: 100vw; height: 100vh;
            background: linear-gradient(180deg, rgba(2,2,5,0.2) 0%, rgba(2,2,5,0.95) 100%);
            z-index: 10; pointer-events: none;
        }}

        .ui-container {{
            position: absolute; top:0; left:0; width:100vw; height:100vh; z-index: 20;
            display: flex; flex-direction: column; padding: 40px; box-sizing: border-box;
        }}

        .header {{
            display: flex; justify-content: space-between; align-items: flex-start;
        }}

        .title {{
            font-size: 32px; font-weight: 800; letter-spacing: 4px; text-transform: uppercase;
            color: {color}; text-shadow: 0 0 20px {color};
        }}

        .subtitle {{
            font-family: 'JetBrains Mono', monospace; font-size: 12px; color: #8b8b9e;
            margin-top: 8px; letter-spacing: 2px;
        }}

        .return-link {{
            color: rgba(255,255,255,0.5); font-family: 'JetBrains Mono', monospace; font-size: 12px;
            text-decoration: none; border: 1px solid rgba(255,255,255,0.2); padding: 8px 16px;
            border-radius: 20px; transition: all 0.3s;
        }}
        .return-link:hover {{ color: {color}; border-color: {color}; background: rgba(255,255,255,0.05); box-shadow: 0 0 15px {color}; }}

        /* Chat / Response Array */
        .response-feed {{
            flex: 1; padding: 20px 0; overflow-y: auto; display: flex; flex-direction: column; gap: 20px; margin-top: 20px;
            scrollbar-width: none;
        }}

        .ai-msg {{
            background: rgba(20,20,30,0.8); border-left: 3px solid {color}; padding: 20px;
            border-radius: 0 12px 12px 0; max-width: 80%; backdrop-filter: blur(10px);
            font-family: 'JetBrains Mono', monospace; font-size: 13px; line-height: 1.6;
            animation: slideIn 0.3s ease forwards;
        }}

        .user-msg {{
            background: rgba(255,255,255,0.05); border-right: 3px solid #fff; padding: 20px;
            border-radius: 12px 0 0 12px; max-width: 80%; align-self: flex-end; backdrop-filter: blur(10px);
            font-family: 'JetBrains Mono', monospace; font-size: 13px; line-height: 1.6; color: #ccc;
            animation: slideInR 0.3s ease forwards;
        }}

        @keyframes slideIn {{ from {{ opacity:0; transform:translateX(-20px); }} to {{ opacity:1; transform:translateX(0); }} }}
        @keyframes slideInR {{ from {{ opacity:0; transform:translateX(20px); }} to {{ opacity:1; transform:translateX(0); }} }}

        /* Input Container */
        .input-wrapper {{
            position: relative; width: 100%; max-width: 900px; margin: 0 auto;
            background: rgba(5,5,10,0.8); border: 1px solid rgba(255,255,255,0.1);
            border-radius: 30px; display: flex; align-items: center; padding: 5px 20px;
            backdrop-filter: blur(20px); transition: all 0.3s;
            box-shadow: 0 20px 40px rgba(0,0,0,0.5);
        }}

        .input-wrapper:focus-within {{
            border-color: {color}; box-shadow: 0 0 30px {color}33, 0 20px 40px rgba(0,0,0,0.5);
        }}

        .query-input {{
            flex: 1; background: transparent; border: none; outline: none; color: #fff;
            font-family: 'Outfit', sans-serif; font-size: 16px; padding: 15px;
        }}

        .send-btn {{
            background: {color}; border: none; outline: none; color: #000;
            width: 40px; height: 40px; border-radius: 50%; display: flex;
            justify-content: center; align-items: center; cursor: pointer;
            font-weight: 800; transition: transform 0.2s;
        }}

        .send-btn:hover {{ transform: scale(1.1); box-shadow: 0 0 15px {color}; }}

    </style>
</head>
<body>

<canvas id="bg-canvas"></canvas>
<div class="glass-layer"></div>

<div class="ui-container">
    <div class="header">
        <div>
            <div class="title">{title}</div>
            <div class="subtitle">{desc}</div>
        </div>
        <a href="../gate/Gate.html" class="return-link">← RETURN TO ARGONATH</a>
    </div>

    <div class="response-feed" id="feed">
        <div class="ai-msg">
            [SYS_READY] Model connected. Establishing invariant constraint... α + ω = 15 locked.<br><br>
            Awaiting input sequence for {title} intelligence parsing.
        </div>
    </div>

    <div class="input-wrapper">
        <span style="color:{color}; font-family:'JetBrains Mono'; margin-right:10px;">></span>
        <input type="text" class="query-input" id="cmd-in" placeholder="Query the system or run a codebase action..." autocomplete="off">
        <button class="send-btn" id="snd-btn">↳</button>
    </div>
</div>

<script>
    // Minimal Background Entropy
    const canvas = document.getElementById('bg-canvas');
    const ctx = canvas.getContext('2d');
    canvas.width = window.innerWidth; canvas.height = window.innerHeight;
    
    let particles = [];
    for(let i=0; i<150; i++) {{
        particles.push({{
            x: Math.random() * canvas.width,
            y: Math.random() * canvas.height,
            vx: (Math.random() - 0.5) * 0.5,
            vy: (Math.random() - 0.5) * 0.5,
            size: Math.random() * 2
        }});
    }}

    function draw() {{
        ctx.clearRect(0,0,canvas.width,canvas.height);
        ctx.fillStyle = '{color}';
        particles.forEach(p => {{
            p.x += p.vx; p.y += p.vy;
            if(p.x < 0 || p.x > canvas.width) p.vx *= -1;
            if(p.y < 0 || p.y > canvas.height) p.vy *= -1;
            ctx.beginPath();
            ctx.arc(p.x, p.y, p.size, 0, Math.PI*2);
            ctx.fill();
        }});
        requestAnimationFrame(draw);
    }}
    draw();

    window.addEventListener('resize', () => {{
        canvas.width = window.innerWidth; canvas.height = window.innerHeight;
    }});

    // AI Response Emulation
    const input = document.getElementById('cmd-in');
    const feed = document.getElementById('feed');
    const sendBtn = document.getElementById('snd-btn');

    function sendMsg() {{
        const txt = input.value.trim();
        if(!txt) return;
        
        // User Msg
        const uDiv = document.createElement('div');
        uDiv.className = 'user-msg';
        uDiv.innerText = txt;
        feed.appendChild(uDiv);
        input.value = '';
        
        feed.scrollTop = feed.scrollHeight;

        // Simul AI processing
        setTimeout(() => {{
            const aiDiv = document.createElement('div');
            aiDiv.className = 'ai-msg';
            
            if (txt.toLowerCase().includes('help')) {{
                aiDiv.innerHTML = `[INFO] Available targets via '{title}':<br><br>- Analyze Topology<br>- RAG Search Vectors<br>- Enforce α + ω = 15<br>- Generate Reforge Matrix`;
            }} else if (txt.toLowerCase().includes('reforge')) {{
                aiDiv.innerHTML = `[PROCESS] Tri-Weavon Reforge engine engaged. Cross-referencing Claude, Grok, and Gemini shards for optimal architectural alignment.`;
            }} else {{
                aiDiv.innerHTML = `[SYNT] Parsed cognitive vector for: "${{txt}}".<br><br>Action mapped to Tri-Weavon Engine. Executing...`;
            }}
            
            feed.appendChild(aiDiv);
            feed.scrollTop = feed.scrollHeight;
            
        }}, 800 + Math.random() * 1000);
    }}

    input.addEventListener('keypress', e => {{ if(e.key === 'Enter') sendMsg(); }});
    sendBtn.addEventListener('click', sendMsg);
</script>
</body>
</html>
"""

def build_endpoints():
    for rel_path, config in ENDPOINTS.items():
        full_path = os.path.join(BASE_DIR, rel_path)
        os.makedirs(os.path.dirname(full_path), exist_ok=True)
        with open(full_path, "w", encoding="utf-8") as f:
            html = TEMPLATE.format(
                title=config['title'],
                color=config['color'],
                desc=config['desc']
            )
            f.write(html)
        print(f"[*] Built {rel_path} endpoint.")

if __name__ == "__main__":
    build_endpoints()
