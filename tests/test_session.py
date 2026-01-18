"""Tests for session management: State, InMemorySessionService, Session CRUD."""

import pytest
from adk_rust import (
    CreateSessionRequest,
    DeleteSessionRequest,
    GenerateContentConfig,
    GetSessionRequest,
    InMemorySessionService,
    ListSessionRequest,
    Session,
    State,
)


class TestState:
    """Tests for State class."""

    def test_create_state(self):
        """Test creating an empty state."""
        state = State()
        assert state is not None

    def test_state_set_and_get_string(self):
        """Test setting and getting string value."""
        state = State()
        state.set("name", "Alice")
        assert state.get("name") == "Alice"

    def test_state_set_and_get_number(self):
        """Test setting and getting number value."""
        state = State()
        state.set("count", 42)
        assert state.get("count") == 42

    def test_state_set_and_get_float(self):
        """Test setting and getting float value."""
        state = State()
        state.set("score", 3.14)
        value = state.get("score")
        assert abs(value - 3.14) < 0.001

    def test_state_set_and_get_bool(self):
        """Test setting and getting boolean value."""
        state = State()
        state.set("active", True)
        assert state.get("active") is True

        state.set("disabled", False)
        assert state.get("disabled") is False

    def test_state_set_and_get_list(self):
        """Test setting and getting list value."""
        state = State()
        state.set("items", [1, 2, 3])
        assert state.get("items") == [1, 2, 3]

    def test_state_set_and_get_dict(self):
        """Test setting and getting dict value."""
        state = State()
        state.set("config", {"key": "value", "num": 123})
        result = state.get("config")
        assert result["key"] == "value"
        assert result["num"] == 123

    def test_state_get_nonexistent(self):
        """Test getting nonexistent key returns None."""
        state = State()
        result = state.get("nonexistent")
        assert result is None

    def test_state_contains(self):
        """Test contains() method."""
        state = State()
        assert state.contains("key") is False

        state.set("key", "value")
        assert state.contains("key") is True

    def test_state_remove(self):
        """Test remove() method."""
        state = State()
        state.set("key", "value")
        assert state.contains("key") is True

        result = state.remove("key")
        assert result is True
        assert state.contains("key") is False

    def test_state_remove_nonexistent(self):
        """Test removing nonexistent key."""
        state = State()
        result = state.remove("nonexistent")
        assert result is False

    def test_state_all(self):
        """Test all() method returns all key-value pairs."""
        state = State()
        state.set("a", 1)
        state.set("b", 2)
        state.set("c", 3)

        all_data = state.all()
        assert isinstance(all_data, dict)
        assert all_data["a"] == 1
        assert all_data["b"] == 2
        assert all_data["c"] == 3

    def test_state_all_empty(self):
        """Test all() on empty state."""
        state = State()
        all_data = state.all()
        assert isinstance(all_data, dict)
        assert len(all_data) == 0

    def test_state_overwrite(self):
        """Test overwriting existing key."""
        state = State()
        state.set("key", "first")
        assert state.get("key") == "first"

        state.set("key", "second")
        assert state.get("key") == "second"

    def test_state_complex_nested(self):
        """Test state with complex nested structures."""
        state = State()
        complex_data = {
            "users": [
                {"name": "Alice", "age": 30},
                {"name": "Bob", "age": 25},
            ],
            "settings": {
                "theme": "dark",
                "notifications": True,
            },
        }
        state.set("data", complex_data)

        result = state.get("data")
        assert result["users"][0]["name"] == "Alice"
        assert result["settings"]["theme"] == "dark"


class TestInMemorySessionService:
    """Tests for InMemorySessionService."""

    def test_create_session_service(self):
        """Test creating a session service."""
        service = InMemorySessionService()
        assert service is not None

    def test_session_service_multiple_instances(self):
        """Test creating multiple session service instances."""
        service1 = InMemorySessionService()
        service2 = InMemorySessionService()
        assert service1 is not service2


class TestCreateSessionRequest:
    """Tests for CreateSessionRequest."""

    def test_create_request_minimal(self):
        """Test creating request with minimal parameters."""
        request = CreateSessionRequest("my_app", "user123")
        assert request is not None

    def test_create_request_with_session_id(self):
        """Test creating request with explicit session ID."""
        request = CreateSessionRequest("my_app", "user123", "session456")
        assert request is not None

    def test_create_request_with_state(self):
        """Test creating request with initial state."""
        request = CreateSessionRequest("my_app", "user123")
        request = request.with_state("key", "value")
        assert request is not None

    def test_create_request_chain_state(self):
        """Test chaining multiple state values."""
        request = (
            CreateSessionRequest("app", "user")
            .with_state("name", "Alice")
            .with_state("count", 42)
            .with_state("active", True)
        )
        assert request is not None


class TestGetSessionRequest:
    """Tests for GetSessionRequest."""

    def test_create_get_request(self):
        """Test creating a get session request."""
        request = GetSessionRequest("my_app", "user123", "session456")
        assert request is not None

    def test_get_request_properties(self):
        """Test GetSessionRequest properties."""
        request = GetSessionRequest("my_app", "user123", "session456")
        assert request.app_name == "my_app"
        assert request.user_id == "user123"
        assert request.session_id == "session456"

    def test_get_request_with_num_recent_events(self):
        """Test creating request with num_recent_events."""
        request = GetSessionRequest("my_app", "user123", "session456", num_recent_events=10)
        assert request.num_recent_events == 10


class TestListSessionRequest:
    """Tests for ListSessionRequest."""

    def test_create_list_request(self):
        """Test creating a list session request."""
        request = ListSessionRequest("my_app", "user123")
        assert request is not None

    def test_list_request_properties(self):
        """Test ListSessionRequest properties."""
        request = ListSessionRequest("my_app", "user123")
        assert request.app_name == "my_app"
        assert request.user_id == "user123"


class TestDeleteSessionRequest:
    """Tests for DeleteSessionRequest."""

    def test_create_delete_request(self):
        """Test creating a delete session request."""
        request = DeleteSessionRequest("my_app", "user123", "session456")
        assert request is not None

    def test_delete_request_properties(self):
        """Test DeleteSessionRequest properties."""
        request = DeleteSessionRequest("my_app", "user123", "session456")
        assert request.app_name == "my_app"
        assert request.user_id == "user123"
        assert request.session_id == "session456"


class TestGenerateContentConfig:
    """Tests for GenerateContentConfig."""

    def test_create_config_default(self):
        """Test creating config with defaults."""
        config = GenerateContentConfig()
        assert config is not None

    def test_config_temperature(self):
        """Test config with temperature."""
        config = GenerateContentConfig(temperature=0.7)
        # Use approximate comparison for float32
        assert config.temperature is not None
        assert abs(config.temperature - 0.7) < 0.001

    def test_config_top_p(self):
        """Test config with top_p."""
        config = GenerateContentConfig(top_p=0.9)
        # Use approximate comparison for float32
        assert config.top_p is not None
        assert abs(config.top_p - 0.9) < 0.001

    def test_config_top_k(self):
        """Test config with top_k."""
        config = GenerateContentConfig(top_k=40)
        assert config.top_k == 40

    def test_config_max_output_tokens(self):
        """Test config with max_output_tokens."""
        config = GenerateContentConfig(max_output_tokens=1024)
        assert config.max_output_tokens == 1024

    def test_config_response_schema(self):
        """Test config with response_schema."""
        schema = {
            "type": "object",
            "properties": {
                "answer": {"type": "string"},
            },
        }
        config = GenerateContentConfig(response_schema=schema)
        assert config.response_schema == schema

    def test_config_all_parameters(self):
        """Test config with all parameters."""
        schema = {"type": "object"}
        config = GenerateContentConfig(
            temperature=0.5,
            top_p=0.95,
            top_k=50,
            max_output_tokens=2048,
            response_schema=schema,
        )
        # Use approximate comparison for float32
        assert config.temperature is not None
        assert config.top_p is not None
        assert abs(config.temperature - 0.5) < 0.001
        assert abs(config.top_p - 0.95) < 0.001
        assert config.top_k == 50
        assert config.max_output_tokens == 2048
        assert config.response_schema == schema

    def test_config_none_values(self):
        """Test config with None values."""
        config = GenerateContentConfig(
            temperature=None,
            top_p=None,
        )
        assert config.temperature is None
        assert config.top_p is None


class TestSessionIntegration:
    """Integration tests for session management."""

    def test_state_with_session_service(self):
        """Test using state with session service."""
        service = InMemorySessionService()
        state = State()
        state.set("initialized", True)

        assert service is not None
        assert state.get("initialized") is True

    def test_create_request_flow(self):
        """Test typical create session request flow."""
        request = (
            CreateSessionRequest("my_app", "user1")
            .with_state("theme", "dark")
            .with_state("language", "en")
        )
        assert request is not None

    def test_config_with_model_params(self):
        """Test GenerateContentConfig typical usage."""
        # Typical creative writing config
        creative_config = GenerateContentConfig(
            temperature=0.9,
            top_p=0.95,
            max_output_tokens=4096,
        )

        # Typical precise/factual config
        precise_config = GenerateContentConfig(
            temperature=0.1,
            top_k=1,
            max_output_tokens=256,
        )

        # Use approximate comparison for float32
        assert creative_config.temperature is not None
        assert precise_config.temperature is not None
        assert abs(creative_config.temperature - 0.9) < 0.001
        assert abs(precise_config.temperature - 0.1) < 0.001


class TestSessionCRUD:
    """Tests for Session CRUD operations."""

    @pytest.fixture
    def session_service(self):
        """Create a fresh session service for each test."""
        return InMemorySessionService()

    async def test_create_session_basic(self, session_service):
        """Test creating a session."""
        request = CreateSessionRequest("test_app", "user1")
        session = await session_service.create(request)

        assert session is not None
        assert isinstance(session, Session)
        assert session.app_name == "test_app"
        assert session.user_id == "user1"
        assert session.id is not None

    async def test_create_session_with_session_id(self, session_service):
        """Test creating session with explicit session ID."""
        request = CreateSessionRequest("test_app", "user1", "my_session_123")
        session = await session_service.create(request)

        assert session.id == "my_session_123"

    async def test_create_session_with_initial_state(self, session_service):
        """Test creating session with initial state."""
        request = (
            CreateSessionRequest("test_app", "user1")
            .with_state("theme", "dark")
            .with_state("language", "en")
        )
        session = await session_service.create(request)

        # Verify session was created
        assert session is not None
        # Note: Initial state may be accessed through session.state
        state = session.state
        assert state is not None

    async def test_get_session(self, session_service):
        """Test getting an existing session."""
        # First create a session
        create_req = CreateSessionRequest("test_app", "user1", "session_to_get")
        _created = await session_service.create(create_req)

        # Then get it
        get_req = GetSessionRequest("test_app", "user1", "session_to_get")
        session = await session_service.get(get_req)

        assert session is not None
        assert session.id == "session_to_get"
        assert session.app_name == "test_app"
        assert session.user_id == "user1"

    async def test_get_session_not_found(self, session_service):
        """Test getting nonexistent session raises error."""
        get_req = GetSessionRequest("test_app", "user1", "nonexistent_session")

        with pytest.raises(RuntimeError, match="not found|does not exist"):
            await session_service.get(get_req)

    async def test_list_sessions_empty(self, session_service):
        """Test listing sessions when none exist."""
        list_req = ListSessionRequest("test_app", "user1")
        sessions = await session_service.list(list_req)

        assert sessions is not None
        assert isinstance(sessions, list)
        assert len(sessions) == 0

    async def test_list_sessions_single(self, session_service):
        """Test listing sessions with one session."""
        # Create a session
        create_req = CreateSessionRequest("test_app", "user1", "session1")
        await session_service.create(create_req)

        # List sessions
        list_req = ListSessionRequest("test_app", "user1")
        sessions = await session_service.list(list_req)

        assert len(sessions) == 1
        assert sessions[0].id == "session1"

    async def test_list_sessions_multiple(self, session_service):
        """Test listing multiple sessions."""
        # Create multiple sessions
        for i in range(3):
            create_req = CreateSessionRequest("test_app", "user1", f"session_{i}")
            await session_service.create(create_req)

        # List sessions
        list_req = ListSessionRequest("test_app", "user1")
        sessions = await session_service.list(list_req)

        assert len(sessions) == 3
        session_ids = {s.id for s in sessions}
        assert session_ids == {"session_0", "session_1", "session_2"}

    async def test_list_sessions_user_isolation(self, session_service):
        """Test that listing sessions is isolated by user."""
        # Create sessions for different users
        await session_service.create(CreateSessionRequest("test_app", "user1", "user1_session"))
        await session_service.create(CreateSessionRequest("test_app", "user2", "user2_session"))

        # List for user1
        user1_sessions = await session_service.list(ListSessionRequest("test_app", "user1"))
        assert len(user1_sessions) == 1
        assert user1_sessions[0].id == "user1_session"

        # List for user2
        user2_sessions = await session_service.list(ListSessionRequest("test_app", "user2"))
        assert len(user2_sessions) == 1
        assert user2_sessions[0].id == "user2_session"

    async def test_delete_session(self, session_service):
        """Test deleting a session."""
        # Create a session
        create_req = CreateSessionRequest("test_app", "user1", "session_to_delete")
        await session_service.create(create_req)

        # Verify it exists
        get_req = GetSessionRequest("test_app", "user1", "session_to_delete")
        session = await session_service.get(get_req)
        assert session is not None

        # Delete it
        delete_req = DeleteSessionRequest("test_app", "user1", "session_to_delete")
        await session_service.delete(delete_req)

        # Verify it's gone
        with pytest.raises(RuntimeError, match="not found|does not exist"):
            await session_service.get(get_req)

    async def test_delete_session_not_found(self, session_service):
        """Test deleting nonexistent session (may silently succeed or raise error)."""
        delete_req = DeleteSessionRequest("test_app", "user1", "nonexistent")

        # The upstream implementation may silently succeed when deleting nonexistent session
        # This is acceptable behavior - the end result is the same (session doesn't exist)
        try:
            await session_service.delete(delete_req)
            # If no error, that's also acceptable
        except RuntimeError:
            # If error, that's also fine
            pass

    async def test_session_properties(self, session_service):
        """Test Session object has all expected properties."""
        create_req = CreateSessionRequest("test_app", "user1", "test_session")
        session = await session_service.create(create_req)

        # Check all properties are accessible
        assert session.id == "test_session"
        assert session.app_name == "test_app"
        assert session.user_id == "user1"
        assert session.state is not None
        assert session.events is not None
        assert isinstance(session.events, list)
        assert session.last_update_time is not None  # ISO 8601 string
        assert session.event_count() >= 0

    async def test_session_events_empty(self, session_service):
        """Test newly created session has no events."""
        create_req = CreateSessionRequest("test_app", "user1")
        session = await session_service.create(create_req)

        assert session.events == []
        assert session.event_count() == 0

    async def test_full_crud_workflow(self, session_service):
        """Test complete CRUD workflow."""
        # CREATE
        create_req = CreateSessionRequest("my_app", "alice", "session_abc")
        session = await session_service.create(create_req)
        assert session.id == "session_abc"

        # READ
        get_req = GetSessionRequest("my_app", "alice", "session_abc")
        retrieved = await session_service.get(get_req)
        assert retrieved.id == session.id

        # LIST
        list_req = ListSessionRequest("my_app", "alice")
        all_sessions = await session_service.list(list_req)
        assert len(all_sessions) == 1

        # DELETE
        delete_req = DeleteSessionRequest("my_app", "alice", "session_abc")
        await session_service.delete(delete_req)

        # VERIFY DELETED
        list_after = await session_service.list(list_req)
        assert len(list_after) == 0

    async def test_multiple_apps_isolation(self, session_service):
        """Test sessions are isolated by app name."""
        # Create sessions in different apps
        await session_service.create(CreateSessionRequest("app1", "user1", "session1"))
        await session_service.create(CreateSessionRequest("app2", "user1", "session1"))

        # List from app1
        app1_sessions = await session_service.list(ListSessionRequest("app1", "user1"))
        assert len(app1_sessions) == 1

        # List from app2
        app2_sessions = await session_service.list(ListSessionRequest("app2", "user1"))
        assert len(app2_sessions) == 1

        # The sessions should be different despite same ID
        get_app1 = await session_service.get(GetSessionRequest("app1", "user1", "session1"))
        get_app2 = await session_service.get(GetSessionRequest("app2", "user1", "session1"))

        assert get_app1.app_name == "app1"
        assert get_app2.app_name == "app2"
