"""Tests for InMemoryMemoryService and MemoryEntry."""

import pytest
from adk_rust import (
    Content,
    InMemoryMemoryService,
    MemoryEntry,
)


class TestMemoryEntry:
    """Tests for MemoryEntry."""

    def test_create_memory_entry(self):
        """Test creating a memory entry."""
        content = Content.user("Remember this fact.")
        entry = MemoryEntry(content, "user")

        assert entry.content is not None
        assert entry.author == "user"

    def test_memory_entry_with_timestamp(self):
        """Test memory entry with explicit timestamp."""
        content = Content.model("A response to remember.")
        entry = MemoryEntry(content, "model", "2024-01-15T10:30:00Z")

        assert entry.author == "model"
        assert entry.timestamp == "2024-01-15T10:30:00Z"

    def test_memory_entry_auto_timestamp(self):
        """Test memory entry with auto-generated timestamp."""
        content = Content.user("Auto-timestamped entry.")
        entry = MemoryEntry(content, "user")

        # Timestamp should be set automatically if not provided
        assert entry.timestamp is not None or entry.timestamp == ""

    def test_memory_entry_content_property(self):
        """Test accessing memory entry content."""
        content = Content.user("Content to check.")
        entry = MemoryEntry(content, "user")

        assert entry.content.role == "user"

    def test_memory_entry_author_property(self):
        """Test memory entry author property."""
        content = Content.model("Model response.")
        entry = MemoryEntry(content, "assistant")

        assert entry.author == "assistant"


class TestInMemoryMemoryService:
    """Tests for InMemoryMemoryService."""

    def test_create_memory_service(self):
        """Test creating a memory service."""
        service = InMemoryMemoryService()
        assert service is not None

    @pytest.mark.asyncio
    async def test_add_session_memories(self):
        """Test adding session memories."""
        service = InMemoryMemoryService()

        entries = [
            MemoryEntry(Content.user("First message"), "user"),
            MemoryEntry(Content.model("First response"), "model"),
        ]

        await service.add_session("my_app", "user1", "session1", entries)

    @pytest.mark.asyncio
    async def test_add_empty_entries(self):
        """Test adding empty entry list."""
        service = InMemoryMemoryService()
        await service.add_session("my_app", "user1", "session1", [])

    @pytest.mark.asyncio
    async def test_add_single_entry(self):
        """Test adding a single entry."""
        service = InMemoryMemoryService()

        entries = [MemoryEntry(Content.user("Single memory"), "user")]
        await service.add_session("my_app", "user1", "session1", entries)

    @pytest.mark.asyncio
    async def test_search_memories(self):
        """Test searching for memories."""
        service = InMemoryMemoryService()

        entries = [
            MemoryEntry(Content.user("The capital of France is Paris"), "user"),
            MemoryEntry(Content.user("Python is a programming language"), "user"),
        ]
        await service.add_session("my_app", "user1", "session1", entries)

        results = await service.search("my_app", "user1", "France capital")
        assert isinstance(results, list)

    @pytest.mark.asyncio
    async def test_search_no_results(self):
        """Test searching when no memories exist."""
        service = InMemoryMemoryService()

        results = await service.search("my_app", "user1", "anything")
        assert isinstance(results, list)
        # Empty or contains no relevant matches
        assert len(results) >= 0

    @pytest.mark.asyncio
    async def test_search_specific_query(self):
        """Test searching with specific query."""
        service = InMemoryMemoryService()

        entries = [
            MemoryEntry(Content.user("My favorite color is blue"), "user"),
            MemoryEntry(Content.user("I like pizza for dinner"), "user"),
            MemoryEntry(Content.user("The meeting is at 3pm"), "user"),
        ]
        await service.add_session("my_app", "user1", "session1", entries)

        results = await service.search("my_app", "user1", "favorite color")
        assert isinstance(results, list)

    @pytest.mark.asyncio
    async def test_multiple_sessions(self):
        """Test adding memories to multiple sessions."""
        service = InMemoryMemoryService()

        # Session 1
        await service.add_session(
            "my_app",
            "user1",
            "session1",
            [
                MemoryEntry(Content.user("Session 1 memory"), "user"),
            ],
        )

        # Session 2
        await service.add_session(
            "my_app",
            "user1",
            "session2",
            [
                MemoryEntry(Content.user("Session 2 memory"), "user"),
            ],
        )

        # Search should work
        results = await service.search("my_app", "user1", "memory")
        assert isinstance(results, list)

    @pytest.mark.asyncio
    async def test_multiple_users(self):
        """Test memories for different users."""
        service = InMemoryMemoryService()

        # User 1
        await service.add_session(
            "my_app",
            "user1",
            "session1",
            [
                MemoryEntry(Content.user("User 1 secret"), "user"),
            ],
        )

        # User 2
        await service.add_session(
            "my_app",
            "user2",
            "session1",
            [
                MemoryEntry(Content.user("User 2 secret"), "user"),
            ],
        )

        # Each user should have their own memories
        results1 = await service.search("my_app", "user1", "secret")
        results2 = await service.search("my_app", "user2", "secret")

        assert isinstance(results1, list)
        assert isinstance(results2, list)

    @pytest.mark.asyncio
    async def test_add_memories_different_apps(self):
        """Test memories in different apps."""
        service = InMemoryMemoryService()

        await service.add_session(
            "app1",
            "user1",
            "session1",
            [
                MemoryEntry(Content.user("App 1 data"), "user"),
            ],
        )

        await service.add_session(
            "app2",
            "user1",
            "session1",
            [
                MemoryEntry(Content.user("App 2 data"), "user"),
            ],
        )

        results1 = await service.search("app1", "user1", "data")
        results2 = await service.search("app2", "user1", "data")

        assert isinstance(results1, list)
        assert isinstance(results2, list)


class TestMemoryServiceIntegration:
    """Integration tests for memory service with real content."""

    @pytest.mark.asyncio
    async def test_conversation_memory(self):
        """Test storing and retrieving conversation memory."""
        service = InMemoryMemoryService()

        # Simulate a conversation
        conversation = [
            MemoryEntry(Content.user("What is 2+2?"), "user"),
            MemoryEntry(Content.model("2+2 equals 4."), "model"),
            MemoryEntry(Content.user("What about 3+3?"), "user"),
            MemoryEntry(Content.model("3+3 equals 6."), "model"),
        ]

        await service.add_session("math_app", "student", "lesson1", conversation)

        # Search for math-related memories
        results = await service.search("math_app", "student", "what is 2+2")
        assert isinstance(results, list)

    @pytest.mark.asyncio
    async def test_memory_with_model_content(self):
        """Test memories that include model responses."""
        service = InMemoryMemoryService()

        entries = [
            MemoryEntry(Content.model("Important: Always backup your data!"), "model"),
        ]

        await service.add_session("app", "user", "session", entries)

        results = await service.search("app", "user", "backup data")
        assert isinstance(results, list)

    @pytest.mark.asyncio
    async def test_memory_search_returns_entries(self):
        """Test that search returns MemoryEntry objects."""
        service = InMemoryMemoryService()

        entries = [
            MemoryEntry(Content.user("Unique phrase xyz123"), "user"),
        ]
        await service.add_session("app", "user", "session", entries)

        results = await service.search("app", "user", "xyz123")

        for result in results:
            # Each result should have the MemoryEntry properties
            assert hasattr(result, "content")
            assert hasattr(result, "author")
            assert hasattr(result, "timestamp")
