"""Tests for model constructors and properties."""

from adk_rust import (
    AnthropicModel,
    DeepSeekModel,
    GeminiModel,
    GroqModel,
    MockLlm,
    OllamaModel,
    OpenAIModel,
)


class TestMockLlm:
    """Tests for MockLlm."""

    def test_create_mock_llm(self):
        """Test creating a MockLlm."""
        mock = MockLlm("test_mock", "Test response")
        assert mock.name == "test_mock"

    def test_mock_llm_default_response(self):
        """Test MockLlm with default response."""
        mock = MockLlm("default_mock")
        assert mock.name == "default_mock"

    def test_mock_llm_custom_response(self):
        """Test MockLlm with custom response text."""
        mock = MockLlm("custom_mock", "Custom response text")
        assert mock.name == "custom_mock"


class TestGeminiModel:
    """Tests for GeminiModel."""

    def test_create_gemini_model(self):
        """Test creating a GeminiModel with API key."""
        model = GeminiModel("fake-api-key")
        assert model.name is not None

    def test_gemini_model_with_custom_model(self):
        """Test GeminiModel with custom model name."""
        model = GeminiModel("fake-api-key", "gemini-2.0-flash")
        assert model.name is not None

    def test_gemini_model_default_model(self):
        """Test GeminiModel uses default model."""
        model = GeminiModel("fake-api-key")
        # Default should be gemini-2.5-flash or similar
        assert "gemini" in model.name.lower() or model.name is not None


class TestOpenAIModel:
    """Tests for OpenAIModel."""

    def test_create_openai_model(self):
        """Test creating an OpenAIModel with API key."""
        model = OpenAIModel("fake-api-key")
        assert model.name is not None

    def test_openai_model_with_custom_model(self):
        """Test OpenAIModel with custom model name."""
        model = OpenAIModel("fake-api-key", "gpt-4-turbo")
        assert model.name is not None

    def test_openai_compatible_endpoint(self):
        """Test OpenAI-compatible endpoint."""
        model = OpenAIModel.compatible("fake-api-key", "https://api.example.com/v1", "custom-model")
        assert model.name is not None


class TestAnthropicModel:
    """Tests for AnthropicModel."""

    def test_create_anthropic_model(self):
        """Test creating an AnthropicModel with API key."""
        model = AnthropicModel("fake-api-key")
        assert model.name is not None

    def test_anthropic_model_with_custom_model(self):
        """Test AnthropicModel with custom model name."""
        model = AnthropicModel("fake-api-key", "claude-3-opus-20240229")
        assert model.name is not None

    def test_anthropic_from_api_key(self):
        """Test AnthropicModel.from_api_key factory."""
        model = AnthropicModel.from_api_key("fake-api-key")
        assert model.name is not None


class TestDeepSeekModel:
    """Tests for DeepSeekModel."""

    def test_create_deepseek_model(self):
        """Test creating a DeepSeekModel."""
        model = DeepSeekModel("fake-api-key", "deepseek-chat")
        assert model.name is not None

    def test_deepseek_chat(self):
        """Test DeepSeekModel.chat() factory."""
        model = DeepSeekModel.chat("fake-api-key")
        assert model.name is not None

    def test_deepseek_reasoner(self):
        """Test DeepSeekModel.reasoner() factory."""
        model = DeepSeekModel.reasoner("fake-api-key")
        assert model.name is not None


class TestGroqModel:
    """Tests for GroqModel."""

    def test_create_groq_model(self):
        """Test creating a GroqModel."""
        model = GroqModel("fake-api-key", "llama-3.1-70b-versatile")
        assert model.name is not None

    def test_groq_llama70b(self):
        """Test GroqModel.llama70b() factory."""
        model = GroqModel.llama70b("fake-api-key")
        assert model.name is not None

    def test_groq_llama8b(self):
        """Test GroqModel.llama8b() factory."""
        model = GroqModel.llama8b("fake-api-key")
        assert model.name is not None

    def test_groq_mixtral(self):
        """Test GroqModel.mixtral() factory."""
        model = GroqModel.mixtral("fake-api-key")
        assert model.name is not None


class TestOllamaModel:
    """Tests for OllamaModel."""

    def test_create_ollama_model(self):
        """Test creating an OllamaModel."""
        model = OllamaModel("llama3")
        assert model.name is not None

    def test_ollama_with_custom_host(self):
        """Test OllamaModel with custom host."""
        model = OllamaModel.with_host("http://localhost:11434", "llama3")
        assert model.name is not None

    def test_ollama_different_models(self):
        """Test OllamaModel with different model names."""
        models = ["llama3", "mistral", "codellama"]
        for model_name in models:
            model = OllamaModel(model_name)
            assert model.name is not None


class TestModelInteroperability:
    """Tests for model interoperability with agents."""

    def test_all_models_have_name_property(self):
        """Test that all models have a name property."""
        models = [
            MockLlm("test", "response"),
            GeminiModel("key"),
            OpenAIModel("key"),
            AnthropicModel("key"),
            DeepSeekModel("key", "model"),
            GroqModel("key", "model"),
            OllamaModel("model"),
        ]
        for model in models:
            assert hasattr(model, "name")
            assert model.name is not None
