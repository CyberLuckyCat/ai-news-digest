import { useState } from 'react';
import '../styles/SourceList.css';

interface Source {
  id: number;
  name: string;
  url: string;
  source_type: string;
  category: string;
  enabled: boolean;
}

interface SourceListProps {
  sources: Source[];
  onAdd: (name: string, url: string, sourceType: string, category: string) => void;
  onDelete: (id: number) => void;
}

function SourceList({ sources, onAdd, onDelete }: SourceListProps) {
  const [showForm, setShowForm] = useState(false);
  const [name, setName] = useState('');
  const [url, setUrl] = useState('');
  const [sourceType, setSourceType] = useState('rss');
  const [category, setCategory] = useState('tech');

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (name && url) {
      onAdd(name, url, sourceType, category);
      setName('');
      setUrl('');
      setShowForm(false);
    }
  };

  return (
    <div className="source-list">
      <div className="source-header">
        <h2>📋 订阅列表</h2>
        <button className="btn-primary" onClick={() => setShowForm(!showForm)}>
          {showForm ? '取消' : '+ 添加订阅'}
        </button>
      </div>

      {showForm && (
        <form className="source-form" onSubmit={handleSubmit}>
          <div className="form-group">
            <label>名称</label>
            <input
              type="text"
              value={name}
              onChange={(e) => setName(e.target.value)}
              placeholder="例如：AI News"
            />
          </div>
          <div className="form-group">
            <label>网址</label>
            <input
              type="url"
              value={url}
              onChange={(e) => setUrl(e.target.value)}
              placeholder="https://..."
            />
          </div>
          <div className="form-group">
            <label>类型</label>
            <select value={sourceType} onChange={(e) => setSourceType(e.target.value)}>
              <option value="rss">RSS</option>
              <option value="website">Website</option>
            </select>
          </div>
          <div className="form-group">
            <label>分类</label>
            <select value={category} onChange={(e) => setCategory(e.target.value)}>
              <option value="ai">AI</option>
              <option value="robot">机器人</option>
              <option value="game">游戏</option>
              <option value="tech">科技</option>
            </select>
          </div>
          <button type="submit" className="btn-primary">添加</button>
        </form>
      )}

      <div className="source-items">
        {sources.length === 0 ? (
          <p className="empty-message">暂无订阅源，点击上方添加按钮添加</p>
        ) : (
          sources.map((source) => (
            <div key={source.id} className="source-item">
              <div className="source-info">
                <h3>{source.name}</h3>
                <p className="source-url">{source.url}</p>
                <div className="source-tags">
                  <span className="tag">{source.source_type}</span>
                  <span className="tag">{source.category}</span>
                </div>
              </div>
              <button
                className="btn-danger"
                onClick={() => onDelete(source.id)}
              >
                删除
              </button>
            </div>
          ))
        )}
      </div>
    </div>
  );
}

export default SourceList;
