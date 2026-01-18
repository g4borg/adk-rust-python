"""Tests for core types: Content, Part, Event."""

import pytest
from adk_rust import Content, Event, Part


class TestContent:
    """Tests for Content type."""

    def test_create_user_content(self):
        """Test creating user content."""
        content = Content.user("Hello, world!")
        assert content.role == "user"
        assert content.get_text() == "Hello, world!"

    def test_create_model_content(self):
        """Test creating model content."""
        content = Content.model("Hi there!")
        assert content.role == "model"
        assert content.get_text() == "Hi there!"

    def test_content_custom_role(self):
        """Test custom content role."""
        content = Content("system")
        assert content.role == "system"

    def test_content_parts_list(self):
        """Test accessing content parts."""
        content = Content.user("Test message")
        parts = content.parts
        assert isinstance(parts, list)
        assert len(parts) >= 1

    def test_content_empty_text(self):
        """Test content with empty text."""
        content = Content.user("")
        assert content.get_text() == ""

    def test_content_multiline_text(self):
        """Test content with multiline text."""
        text = "Line 1\nLine 2\nLine 3"
        content = Content.user(text)
        assert content.get_text() == text


class TestPart:
    """Tests for Part type."""

    def test_create_text_part(self):
        """Test creating a text part."""
        part = Part.text("Hello")
        assert part.is_text()
        assert not part.is_function_call()
        assert part.get_text() == "Hello"

    def test_part_empty_text(self):
        """Test part with empty text."""
        part = Part.text("")
        assert part.is_text()
        assert part.get_text() == ""

    def test_part_unicode_text(self):
        """Test part with unicode text."""
        text = "Hello, world!"
        part = Part.text(text)
        assert part.get_text() == text

    def test_part_special_characters(self):
        """Test part with special characters."""
        text = "Special chars: <>&\"'`~!@#$%^&*()"
        part = Part.text(text)
        assert part.get_text() == text

    def test_part_is_not_function_call(self):
        """Test that text part is not a function call."""
        part = Part.text("Not a function call")
        assert not part.is_function_call()


class TestEvent:
    """Tests for Event type."""

    def test_event_has_id(self, user_content):
        """Test that Event has an id property."""
        # Events are typically created by the runner, not directly
        # We test the interface here
        assert hasattr(Event, "id")

    def test_event_has_author(self):
        """Test that Event has an author property."""
        assert hasattr(Event, "author")

    def test_event_has_content_property(self):
        """Test that Event has content property."""
        assert hasattr(Event, "content")

    def test_event_has_get_text_method(self):
        """Test that Event has get_text() method."""
        assert callable(getattr(Event, "get_text", None))

    def test_event_has_get_state_delta_method(self):
        """Test that Event has get_state_delta() method."""
        assert callable(getattr(Event, "get_state_delta", None))

    def test_event_has_transfer_to_agent_property(self):
        """Test that Event has transfer_to_agent property."""
        assert hasattr(Event, "transfer_to_agent")


class TestContentIntegration:
    """Integration tests for Content with Parts."""

    def test_content_parts_contain_text(self):
        """Test that content parts contain the text."""
        content = Content.user("Test message")
        parts = content.parts

        # Find a text part
        text_parts = [p for p in parts if p.is_text()]
        assert len(text_parts) > 0
        assert text_parts[0].get_text() == "Test message"

    def test_model_content_parts(self):
        """Test model content parts."""
        content = Content.model("Model response")
        parts = content.parts

        text_parts = [p for p in parts if p.is_text()]
        assert len(text_parts) > 0
        assert text_parts[0].get_text() == "Model response"
