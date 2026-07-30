import React from 'react';
import { CrucixTDA_Map } from './components/CrucixTDA_Map';
import { GitNexusGraph } from './components/GitNexusGraph';
import { A2UI_AgentRenderer } from './components/A2UI_AgentRenderer';
import './App.css';

function App() {
  return (
    <div className="coherence-lattice" style={{ display: 'grid', gridTemplateColumns: '1fr 3fr 1fr', gap: '1rem', height: '100vh', backgroundColor: '#050505', color: '#eee', padding: '1rem', boxSizing: 'border-box' }}>
      
      {/* LEFT PANEL: Sensor Grid */}
      <aside className="left-panel" style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
        <h2 style={{ color: '#0f8', margin: 0 }}>SENSOR GRID</h2>
        <div className="metric" style={{ background: '#111', padding: '1rem', borderLeft: '4px solid #0f8' }}>
          <h4>Anyon Pipeline</h4>
          <p>Flow: Stable</p>
        </div>
        <div className="metric" style={{ background: '#111', padding: '1rem', borderLeft: '4px solid #0f8' }}>
          <h4>MoC Queries</h4>
          <p>Rate: 144 req/s</p>
        </div>
      </aside>

      {/* CENTER PANEL: Main Crucix/Shadowbroker Mapping */}
      <main className="center-panel" style={{ display: 'flex', flexDirection: 'column' }}>
        <CrucixTDA_Map />
      </main>

      {/* RIGHT PANEL: Code Intelligence & AI Agent Streams */}
      <aside className="right-panel" style={{ display: 'flex', flexDirection: 'column', gap: '1rem' }}>
        <GitNexusGraph />
        <A2UI_AgentRenderer />
      </aside>

    </div>
  );
}

export default App;
