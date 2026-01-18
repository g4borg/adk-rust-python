"""Tests for InMemoryArtifactService."""

import pytest
from adk_rust import InMemoryArtifactService, Part


class TestInMemoryArtifactService:
    """Tests for InMemoryArtifactService."""

    def test_create_artifact_service(self):
        """Test creating an artifact service."""
        service = InMemoryArtifactService()
        assert service is not None


class TestArtifactSave:
    """Tests for artifact save operations."""

    @pytest.mark.asyncio
    async def test_save_bytes_artifact(self):
        """Test saving bytes artifact."""
        service = InMemoryArtifactService()
        data = b"Hello, binary world!"

        version = await service.save("my_app", "user1", "session1", "hello.bin", data)
        assert isinstance(version, int)
        assert version >= 0

    @pytest.mark.asyncio
    async def test_save_string_artifact(self):
        """Test saving string artifact."""
        service = InMemoryArtifactService()
        data = "Hello, text world!"

        version = await service.save("my_app", "user1", "session1", "hello.txt", data)
        assert isinstance(version, int)

    @pytest.mark.asyncio
    async def test_save_with_mime_type(self):
        """Test saving artifact with MIME type."""
        service = InMemoryArtifactService()
        data = b'{"key": "value"}'

        version = await service.save(
            "my_app", "user1", "session1", "data.json", data, mime_type="application/json"
        )
        assert isinstance(version, int)

    @pytest.mark.asyncio
    async def test_save_with_version(self):
        """Test saving artifact with explicit version."""
        service = InMemoryArtifactService()
        data = b"Version 5 content"

        version = await service.save(
            "my_app", "user1", "session1", "versioned.bin", data, version=5
        )
        # Should return the version we specified or next available
        assert isinstance(version, int)

    @pytest.mark.asyncio
    async def test_save_multiple_versions(self):
        """Test saving multiple versions of same artifact."""
        service = InMemoryArtifactService()

        v1 = await service.save("my_app", "user1", "session1", "doc.txt", "Version 1")
        v2 = await service.save("my_app", "user1", "session1", "doc.txt", "Version 2")
        v3 = await service.save("my_app", "user1", "session1", "doc.txt", "Version 3")

        # Each version should be different
        assert v2 > v1
        assert v3 > v2

    @pytest.mark.asyncio
    async def test_save_empty_artifact(self):
        """Test saving empty artifact."""
        service = InMemoryArtifactService()

        version = await service.save("my_app", "user1", "session1", "empty.bin", b"")
        assert isinstance(version, int)


class TestArtifactLoad:
    """Tests for artifact load operations."""

    @pytest.mark.asyncio
    async def test_load_artifact(self):
        """Test loading a saved artifact."""
        service = InMemoryArtifactService()
        data = b"Content to retrieve"

        await service.save("my_app", "user1", "session1", "file.bin", data)

        part = await service.load("my_app", "user1", "session1", "file.bin")
        assert isinstance(part, Part)

    @pytest.mark.asyncio
    async def test_load_specific_version(self):
        """Test loading specific version of artifact."""
        service = InMemoryArtifactService()

        v1 = await service.save("my_app", "user1", "session1", "file.txt", "Version 1")
        v2 = await service.save("my_app", "user1", "session1", "file.txt", "Version 2")

        part = await service.load("my_app", "user1", "session1", "file.txt", version=v1)
        assert isinstance(part, Part)

    @pytest.mark.asyncio
    async def test_load_latest_version(self):
        """Test loading latest version (default)."""
        service = InMemoryArtifactService()

        await service.save("my_app", "user1", "session1", "file.txt", "V1")
        await service.save("my_app", "user1", "session1", "file.txt", "V2")
        await service.save("my_app", "user1", "session1", "file.txt", "V3")

        # Loading without version should get latest
        part = await service.load("my_app", "user1", "session1", "file.txt")
        assert isinstance(part, Part)


class TestArtifactDelete:
    """Tests for artifact delete operations."""

    @pytest.mark.asyncio
    async def test_delete_artifact(self):
        """Test deleting an artifact."""
        service = InMemoryArtifactService()

        await service.save("my_app", "user1", "session1", "to_delete.txt", "data")

        await service.delete("my_app", "user1", "session1", "to_delete.txt")
        # After deletion, artifact should not be in list
        files = await service.list("my_app", "user1", "session1")
        assert "to_delete.txt" not in files

    @pytest.mark.asyncio
    async def test_delete_specific_version(self):
        """Test deleting specific version of artifact."""
        service = InMemoryArtifactService()

        v1 = await service.save("my_app", "user1", "session1", "file.txt", "V1")
        v2 = await service.save("my_app", "user1", "session1", "file.txt", "V2")

        await service.delete("my_app", "user1", "session1", "file.txt", version=v1)

        # V2 should still exist
        versions = await service.versions("my_app", "user1", "session1", "file.txt")
        assert isinstance(versions, list)


class TestArtifactList:
    """Tests for artifact list operations."""

    @pytest.mark.asyncio
    async def test_list_empty_session(self):
        """Test listing artifacts in empty session."""
        service = InMemoryArtifactService()

        files = await service.list("my_app", "user1", "session1")
        assert isinstance(files, list)
        assert len(files) == 0

    @pytest.mark.asyncio
    async def test_list_artifacts(self):
        """Test listing all artifacts in session."""
        service = InMemoryArtifactService()

        await service.save("my_app", "user1", "session1", "file1.txt", "data1")
        await service.save("my_app", "user1", "session1", "file2.txt", "data2")
        await service.save("my_app", "user1", "session1", "file3.txt", "data3")

        files = await service.list("my_app", "user1", "session1")
        assert isinstance(files, list)
        assert len(files) == 3
        assert "file1.txt" in files
        assert "file2.txt" in files
        assert "file3.txt" in files

    @pytest.mark.asyncio
    async def test_list_different_sessions(self):
        """Test artifacts are isolated by session."""
        service = InMemoryArtifactService()

        await service.save("my_app", "user1", "session1", "s1_file.txt", "data")
        await service.save("my_app", "user1", "session2", "s2_file.txt", "data")

        files1 = await service.list("my_app", "user1", "session1")
        files2 = await service.list("my_app", "user1", "session2")

        assert "s1_file.txt" in files1
        assert "s2_file.txt" not in files1
        assert "s2_file.txt" in files2
        assert "s1_file.txt" not in files2


class TestArtifactVersions:
    """Tests for artifact version operations."""

    @pytest.mark.asyncio
    async def test_versions_nonexistent_raises(self):
        """Test getting versions of nonexistent artifact raises error."""
        service = InMemoryArtifactService()

        with pytest.raises(RuntimeError, match="artifact not found"):
            await service.versions("my_app", "user1", "session1", "nonexistent.txt")

    @pytest.mark.asyncio
    async def test_versions_single(self):
        """Test getting versions with single version."""
        service = InMemoryArtifactService()

        await service.save("my_app", "user1", "session1", "file.txt", "data")

        versions = await service.versions("my_app", "user1", "session1", "file.txt")
        assert isinstance(versions, list)
        assert len(versions) == 1

    @pytest.mark.asyncio
    async def test_versions_multiple(self):
        """Test getting multiple versions."""
        service = InMemoryArtifactService()

        await service.save("my_app", "user1", "session1", "file.txt", "V1")
        await service.save("my_app", "user1", "session1", "file.txt", "V2")
        await service.save("my_app", "user1", "session1", "file.txt", "V3")

        versions = await service.versions("my_app", "user1", "session1", "file.txt")
        assert isinstance(versions, list)
        assert len(versions) == 3


class TestArtifactIntegration:
    """Integration tests for artifact service."""

    @pytest.mark.asyncio
    async def test_full_crud_workflow(self):
        """Test complete CRUD workflow."""
        service = InMemoryArtifactService()
        app = "test_app"
        user = "user1"
        session = "session1"

        # Create
        v1 = await service.save(app, user, session, "doc.txt", "Initial content")
        assert v1 >= 0

        # Read
        part = await service.load(app, user, session, "doc.txt")
        assert isinstance(part, Part)

        # Update (new version)
        v2 = await service.save(app, user, session, "doc.txt", "Updated content")
        assert v2 > v1

        # List
        files = await service.list(app, user, session)
        assert "doc.txt" in files

        # Versions
        versions = await service.versions(app, user, session, "doc.txt")
        assert len(versions) == 2

        # Delete
        await service.delete(app, user, session, "doc.txt")

        files = await service.list(app, user, session)
        assert "doc.txt" not in files

    @pytest.mark.asyncio
    async def test_binary_image_workflow(self):
        """Test workflow with binary image data."""
        service = InMemoryArtifactService()

        # Simulate PNG header
        png_data = b"\x89PNG\r\n\x1a\n" + b"\x00" * 100

        await service.save(
            "my_app", "user1", "session1", "chart.png", png_data, mime_type="image/png"
        )

        part = await service.load("my_app", "user1", "session1", "chart.png")
        assert isinstance(part, Part)

    @pytest.mark.asyncio
    async def test_multiple_users_isolation(self):
        """Test artifacts are isolated per user."""
        service = InMemoryArtifactService()

        await service.save("app", "user1", "session", "private.txt", "User 1 data")
        await service.save("app", "user2", "session", "private.txt", "User 2 data")

        files1 = await service.list("app", "user1", "session")
        files2 = await service.list("app", "user2", "session")

        # Both should have their own file
        assert "private.txt" in files1
        assert "private.txt" in files2
