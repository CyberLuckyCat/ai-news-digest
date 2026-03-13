import '../styles/NewsList.css';

interface News {
  id: number;
  title: string;
  content: string;
  summary: string | null;
  url: string;
  category: string;
  published_at: string;
}

interface NewsListProps {
  news: News[];
  loading: boolean;
  onRefresh: () => void;
  onTriggerCollect: () => void;
}

function NewsList({ news, loading, onRefresh, onTriggerCollect }: NewsListProps) {
  const getCategoryIcon = (category: string) => {
    switch (category) {
      case 'ai': return '🤖';
      case 'robot': return '🦾';
      case 'game': return '🎮';
      case 'tech': return '💻';
      default: return '📰';
    }
  };

  const formatDate = (dateStr: string) => {
    const date = new Date(dateStr);
    return date.toLocaleDateString('zh-CN', {
      year: 'numeric',
      month: 'short',
      day: 'numeric',
      hour: '2-digit',
      minute: '2-digit'
    });
  };

  return (
    <div className="news-list">
      <div className="news-header">
        <h2>📰 新闻摘要</h2>
        <div className="news-actions">
          <button
            className="btn-primary"
            onClick={onTriggerCollect}
            disabled={loading}
          >
            {loading ? '采集中...' : '🔄 立即采集'}
          </button>
          <button
            className="btn-secondary"
            onClick={onRefresh}
            disabled={loading}
          >
            刷新
          </button>
        </div>
      </div>

      {loading && (
        <div className="loading">加载中...</div>
      )}

      <div className="news-items">
        {news.length === 0 && !loading ? (
          <p className="empty-message">暂无新闻，点击上方按钮开始采集</p>
        ) : (
          news.map((item) => (
            <div key={item.id} className="news-item">
              <div className="news-category">
                {getCategoryIcon(item.category)} {item.category}
              </div>
              <h3 className="news-title">
                <a href={item.url} target="_blank" rel="noopener noreferrer">
                  {item.title}
                </a>
              </h3>
              {item.summary && (
                <p className="news-summary">{item.summary}</p>
              )}
              <div className="news-meta">
                <span className="news-date">{formatDate(item.published_at)}</span>
              </div>
            </div>
          ))
        )}
      </div>
    </div>
  );
}

export default NewsList;
