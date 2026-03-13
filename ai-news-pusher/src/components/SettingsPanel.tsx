import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import '../styles/SettingsPanel.css';

interface Settings {
  ai_provider: string;
  api_key: string;
  model: string;
  push_channels: string[];
  channel_targets: Record<string, string>;  // 渠道目标地址，如 email, telegram 等
  timezone: string;
}

// 模型选项配置
const MODEL_OPTIONS: Record<string, { value: string; label: string }[]> = {
  openai: [
    { value: 'gpt-5.1', label: 'GPT-5.1' },
    { value: 'gpt-5.1-codex', label: 'GPT-5.1 Codex' },
    { value: 'gpt-5.1-codex-max', label: 'GPT-5.1 Codex Max' },
    { value: 'gpt-5.1-codex-mini', label: 'GPT-5.1 Codex Mini' },
    { value: 'gpt-5.2', label: 'GPT-5.2' },
    { value: 'gpt-5.2-chat-latest', label: 'GPT-5.2 Chat Latest' },
    { value: 'gpt-5.2-codex', label: 'GPT-5.2 Codex' },
    { value: 'gpt-5.3-chat-latest', label: 'GPT-5.3 Chat Latest' },
    { value: 'gpt-5.3-codex', label: 'GPT-5.3 Codex' },
    { value: 'gpt-5.4', label: 'GPT-5.4' },
  ],
  anthropic: [
    { value: 'claude-3-haiku-20240307', label: 'Claude 3 Haiku (2024-03-07)' },
    { value: 'claude-3-opus-20240229', label: 'Claude 3 Opus (2024-02-29)' },
    { value: 'claude-haiku-4-5-20251001', label: 'Claude Haiku 4 (2025-10-01)' },
    { value: 'claude-opus-4-1-20250805', label: 'Claude Opus 4 (2025-08-05)' },
    { value: 'claude-opus-4-20250514', label: 'Claude Opus 4 (2025-05-14)' },
    { value: 'claude-opus-4-5-20251101', label: 'Claude Opus 4 (2025-11-01)' },
    { value: 'claude-opus-4-6', label: 'Claude Opus 4-6' },
    { value: 'claude-sonnet-4-20250514', label: 'Claude Sonnet 4 (2025-05-14)' },
    { value: 'claude-sonnet-4-5', label: 'Claude Sonnet 4-5' },
    { value: 'claude-sonnet-4-6', label: 'Claude Sonnet 4-6' },
  ],
  minimax: [
    { value: 'MiniMax-M2', label: 'MiniMax M2' },
    { value: 'MiniMax-M2-Stable', label: 'MiniMax M2 Stable' },
    { value: 'MiniMax-M2.1', label: 'MiniMax M2.1' },
    { value: 'MiniMax-M2.5', label: 'MiniMax M2.5' },
  ],
  moonshot: [
    { value: 'kimi-k2-0711-preview', label: 'Kimi K2 (0711 Preview)' },
    { value: 'kimi-k2-0905-preview', label: 'Kimi K2 (0905 Preview)' },
    { value: 'kimi-k2-thinking', label: 'Kimi K2 Thinking' },
    { value: 'kimi-k2-turbo-preview', label: 'Kimi K2 Turbo Preview' },
    { value: 'kimi-k2.5', label: 'Kimi K2.5' },
  ],
  gemini: [
    { value: 'gemini-2.5-flash', label: 'Gemini 2.5 Flash' },
    { value: 'gemini-2.5-flash-lite-preview-09-2025', label: 'Gemini 2.5 Flash-Lite (09-2025)' },
    { value: 'gemini-2.5-flash-preview-09-2025', label: 'Gemini 2.5 Flash Preview (09-2025)' },
    { value: 'gemini-2.5-pro', label: 'Gemini 2.5 Pro' },
    { value: 'gemini-2.5-pro-preview-03-25', label: 'Gemini 2.5 Pro (03-25)' },
    { value: 'gemini-2.5-pro-preview-05-06', label: 'Gemini 2.5 Pro (05-06)' },
    { value: 'gemini-2.5-pro-preview-06-05', label: 'Gemini 2.5 Pro (06-05)' },
    { value: 'gemini-3-flash-preview', label: 'Gemini 3 Flash Preview' },
    { value: 'gemini-3-pro-preview', label: 'Gemini 3 Pro Preview' },
    { value: 'gemini-3.1-pro-preview', label: 'Gemini 3.1 Pro Preview' },
    { value: 'gemini-3.1-pro-preview-customtools', label: 'Gemini 3.1 Pro (Custom Tools)' },
  ],
};

function SettingsPanel() {
  const [settings, setSettings] = useState<Settings>({
    ai_provider: 'openai',
    api_key: '',
    model: 'gpt-4o-mini',
    push_channels: [],
    channel_targets: {},
    timezone: 'Asia/Shanghai'
  });
  const [saving, setSaving] = useState(false);
  const [message, setMessage] = useState('');

  // 获取当前提供商对应的模型列表
  const currentModels = MODEL_OPTIONS[settings.ai_provider] || [
    { value: settings.model, label: settings.model }
  ];

  // 加载设置
  useEffect(() => {
    const loadSettings = async () => {
      try {
        const data = await invoke<Settings>('get_settings');
        setSettings(data);
      } catch (error) {
        console.error('加载设置失败:', error);
      }
    };
    loadSettings();
  }, []);

  // 保存设置
  const handleSave = async () => {
    try {
      setSaving(true);
      await invoke('save_settings', { settings });
      setMessage('设置已保存');
      setTimeout(() => setMessage(''), 3000);
    } catch (error) {
      console.error('保存设置失败:', error);
      setMessage('保存失败');
    } finally {
      setSaving(false);
    }
  };

  const handleChange = (field: keyof Settings, value: string | string[]) => {
    // 当 AI 提供商变更时，自动选择该提供商第一个模型
    if (field === 'ai_provider' && typeof value === 'string') {
      const providerModels = MODEL_OPTIONS[value];
      if (providerModels && providerModels.length > 0) {
        setSettings(prev => ({ ...prev, ai_provider: value, model: providerModels[0].value }));
        return;
      } else {
        // 如果提供商不在列表中，保持原模型
        setSettings(prev => ({ ...prev, ai_provider: value }));
        return;
      }
    }
    setSettings(prev => ({ ...prev, [field]: value }));
  };

  // 处理渠道目标地址变更
  const handleChannelTargetChange = (channel: string, value: string) => {
    setSettings(prev => ({
      ...prev,
      channel_targets: {
        ...prev.channel_targets,
        [channel]: value
      }
    }));
  };

  return (
    <div className="settings-panel">
      <h2>⚙️ 设置</h2>

      {message && <div className="message">{message}</div>}

      <div className="settings-section">
        <h3>AI 配置</h3>
        <div className="form-group">
          <label>AI 提供商</label>
          <select
            value={settings.ai_provider}
            onChange={(e) => handleChange('ai_provider', e.target.value)}
          >
            <option value="openai">OpenAI</option>
            <option value="anthropic">Anthropic (Claude)</option>
            <option value="minimax">MiniMax</option>
            <option value="moonshot">月之暗面 (MoonShot)</option>
            <option value="gemini">Google Gemini</option>
          </select>
        </div>
        <div className="form-group">
          <label>API Key</label>
          <input
            type="password"
            value={settings.api_key}
            onChange={(e) => handleChange('api_key', e.target.value)}
            placeholder="输入 API Key"
          />
        </div>
        <div className="form-group">
          <label>模型</label>
          <select
            value={settings.model}
            onChange={(e) => handleChange('model', e.target.value)}
          >
            {currentModels.map((model) => (
              <option key={model.value} value={model.value}>
                {model.label}
              </option>
            ))}
          </select>
        </div>
      </div>

      <div className="settings-section">
        <h3>推送渠道</h3>
        <div className="channel-options">
          <label className="checkbox-label">
            <input
              type="checkbox"
              checked={settings.push_channels.includes('email')}
              onChange={(e) => {
                const channels = e.target.checked
                  ? [...settings.push_channels, 'email']
                  : settings.push_channels.filter(c => c !== 'email');
                handleChange('push_channels', channels);
              }}
            />
            📧 邮件
          </label>
          {settings.push_channels.includes('email') && (
            <div className="channel-target-input">
              <input
                type="email"
                value={settings.channel_targets.email || ''}
                onChange={(e) => handleChannelTargetChange('email', e.target.value)}
                placeholder="输入收件人邮箱地址"
              />
            </div>
          )}
          
          <label className="checkbox-label">
            <input
              type="checkbox"
              checked={settings.push_channels.includes('telegram')}
              onChange={(e) => {
                const channels = e.target.checked
                  ? [...settings.push_channels, 'telegram']
                  : settings.push_channels.filter(c => c !== 'telegram');
                handleChange('push_channels', channels);
              }}
            />
            ✈️ Telegram
          </label>
          {settings.push_channels.includes('telegram') && (
            <div className="channel-target-input">
              <input
                type="text"
                value={settings.channel_targets.telegram || ''}
                onChange={(e) => handleChannelTargetChange('telegram', e.target.value)}
                placeholder="输入 Telegram Chat ID"
              />
              <a 
                href="https://t.me/ai_news_pusher_bot" 
                target="_blank" 
                rel="noopener noreferrer"
                className="channel-link"
              >
                打开 Telegram Bot →
              </a>
            </div>
          )}
          
          <label className="checkbox-label">
            <input
              type="checkbox"
              checked={settings.push_channels.includes('wechat')}
              onChange={(e) => {
                const channels = e.target.checked
                  ? [...settings.push_channels, 'wechat']
                  : settings.push_channels.filter(c => c !== 'wechat');
                handleChange('push_channels', channels);
              }}
            />
            💬 微信 (预留)
          </label>
          
          <label className="checkbox-label">
            <input
              type="checkbox"
              checked={settings.push_channels.includes('feishu')}
              onChange={(e) => {
                const channels = e.target.checked
                  ? [...settings.push_channels, 'feishu']
                  : settings.push_channels.filter(c => c !== 'feishu');
                handleChange('push_channels', channels);
              }}
            />
            🏢 飞书 (预留)
          </label>
        </div>
      </div>

      <div className="settings-section">
        <h3>其他设置</h3>
        <div className="form-group">
          <label>时区</label>
          <select
            value={settings.timezone}
            onChange={(e) => handleChange('timezone', e.target.value)}
          >
            <option value="Asia/Shanghai">Asia/Shanghai (UTC+8)</option>
            <option value="America/New_York">America/New_York (UTC-5)</option>
            <option value="Europe/London">Europe/London (UTC+0)</option>
          </select>
        </div>
      </div>

      <button
        className="btn-primary save-btn"
        onClick={handleSave}
        disabled={saving}
      >
        {saving ? '保存中...' : '保存设置'}
      </button>
    </div>
  );
}

export default SettingsPanel;
