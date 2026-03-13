import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import SourceList from './components/SourceList';
import NewsList from './components/NewsList';
import SettingsPanel from './components/SettingsPanel';
import './styles/App.css';

// 类型定义
interface Source {
  id: number;
  name: string;
  url: string;
  source_type: string;
  category: string;
  enabled: boolean;
}

interface News {
  id: number;
  title: string;
  content: string;
  summary: string | null;
  url: string;
  category: string;
  published_at: string;
}

function App() {
  const [activeTab, setActiveTab] = useState<'sources' | 'news' | 'settings'>('sources');
  const [sources, setSources] = useState<Source[]>([]);
  const [news, setNews] = useState<News[]>([]);
  const [loading, setLoading] = useState(false);

  // 加载订阅源
  const loadSources = async () => {
    try {
      const data = await invoke<Source[]>('get_sources');
      setSources(data);
    } catch (error) {
      console.error('加载订阅源失败:', error);
    }
  };

  // 加载新闻
  const loadNews = async () => {
    try {
      setLoading(true);
      const data = await invoke<News[]>('get_news', { limit: 50, offset: 0 });
      setNews(data);
    } catch (error) {
      console.error('加载新闻失败:', error);
    } finally {
      setLoading(false);
    }
  };

  // 添加订阅源
  const addSource = async (name: string, url: string, sourceType: string, category: string) => {
    try {
      await invoke('add_source', {
        request: { name, url, source_type: sourceType, category }
      });
      await loadSources();
    } catch (error) {
      console.error('添加订阅源失败:', error);
    }
  };

  // 删除订阅源
  const deleteSource = async (id: number) => {
    try {
      await invoke('delete_source', { id });
      await loadSources();
    } catch (error) {
      console.error('删除订阅源失败:', error);
    }
  };

  // 手动触发采集
  const triggerCollect = async () => {
    try {
      setLoading(true);
      await invoke('trigger_collect');
      await loadNews();
    } catch (error) {
      console.error('触发采集失败:', error);
    } finally {
      setLoading(false);
    }
  };

  // 初始化加载
  useEffect(() => {
    loadSources();
    loadNews();
  }, []);

  return (
    <div className="app">
      <header className="app-header">
        <h1>🤖 AI News Pusher</h1>
        <p>每日 AI、科技、机器人、游戏资讯推送</p>
      </header>

      <nav className="tab-nav">
        <button
          className={activeTab === 'sources' ? 'active' : ''}
          onClick={() => setActiveTab('sources')}
        >
          📋 订阅列表
        </button>
        <button
          className={activeTab === 'news' ? 'active' : ''}
          onClick={() => setActiveTab('news')}
        >
          📰 新闻摘要
        </button>
        <button
          className={activeTab === 'settings' ? 'active' : ''}
          onClick={() => setActiveTab('settings')}
        >
          ⚙️ 设置
        </button>
      </nav>

      <main className="app-content">
        {activeTab === 'sources' && (
          <SourceList
            sources={sources}
            onAdd={addSource}
            onDelete={deleteSource}
          />
        )}

        {activeTab === 'news' && (
          <NewsList
            news={news}
            loading={loading}
            onRefresh={loadNews}
            onTriggerCollect={triggerCollect}
          />
        )}

        {activeTab === 'settings' && (
          <SettingsPanel />
        )}
      </main>
    </div>
  );
}

export default App;
