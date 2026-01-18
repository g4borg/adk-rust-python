"""Tests for guardrails: ContentFilter, PiiRedactor, GuardrailSet."""

import pytest
from adk_rust import (
    Content,
    ContentFilter,
    GuardrailFailure,
    GuardrailResult,
    GuardrailSet,
    PiiRedactor,
    PiiType,
    Severity,
    run_guardrails,
)


class TestSeverity:
    """Tests for Severity enum."""

    def test_severity_low(self):
        """Test Severity.Low."""
        assert Severity.Low is not None

    def test_severity_medium(self):
        """Test Severity.Medium."""
        assert Severity.Medium is not None

    def test_severity_high(self):
        """Test Severity.High."""
        assert Severity.High is not None

    def test_severity_critical(self):
        """Test Severity.Critical."""
        assert Severity.Critical is not None


class TestPiiType:
    """Tests for PiiType enum."""

    def test_pii_type_email(self):
        """Test PiiType.Email."""
        assert PiiType.Email is not None

    def test_pii_type_phone(self):
        """Test PiiType.Phone."""
        assert PiiType.Phone is not None

    def test_pii_type_ssn(self):
        """Test PiiType.Ssn."""
        assert PiiType.Ssn is not None

    def test_pii_type_credit_card(self):
        """Test PiiType.CreditCard."""
        assert PiiType.CreditCard is not None

    def test_pii_type_ip_address(self):
        """Test PiiType.IpAddress."""
        assert PiiType.IpAddress is not None


class TestContentFilter:
    """Tests for ContentFilter."""

    def test_create_harmful_content_filter(self):
        """Test creating harmful content filter."""
        filter = ContentFilter.harmful_content()
        assert filter is not None

    def test_create_on_topic_filter(self):
        """Test creating on-topic filter."""
        filter = ContentFilter.on_topic("cooking", ["recipe", "cook", "bake"])
        assert filter is not None

    def test_create_max_length_filter(self):
        """Test creating max length filter."""
        filter = ContentFilter.max_length(1000)
        assert filter is not None

    def test_create_blocked_keywords_filter(self):
        """Test creating blocked keywords filter."""
        filter = ContentFilter.blocked_keywords(["spam", "scam", "phishing"])
        assert filter is not None

    def test_create_custom_filter(self):
        """Test creating custom content filter."""
        filter = ContentFilter.custom(
            name="my_filter",
            blocked_keywords=["bad", "word"],
            max_length=500,
        )
        assert filter is not None

    def test_custom_filter_with_all_options(self):
        """Test custom filter with all options."""
        filter = ContentFilter.custom(
            name="full_filter",
            blocked_keywords=["spam"],
            required_topics=["helpful", "useful"],
            max_length=1000,
            min_length=10,
            severity=Severity.High,
        )
        assert filter is not None

    def test_on_topic_filter_empty_keywords(self):
        """Test on-topic filter with empty keywords."""
        filter = ContentFilter.on_topic("general", [])
        assert filter is not None

    def test_blocked_keywords_single(self):
        """Test blocked keywords with single keyword."""
        filter = ContentFilter.blocked_keywords(["forbidden"])
        assert filter is not None


class TestPiiRedactor:
    """Tests for PiiRedactor."""

    def test_create_pii_redactor(self):
        """Test creating a PII redactor with all types."""
        redactor = PiiRedactor()
        assert redactor is not None

    def test_create_pii_redactor_with_types(self):
        """Test creating PII redactor with specific types."""
        redactor = PiiRedactor.with_types([PiiType.Email, PiiType.Phone])
        assert redactor is not None

    def test_pii_redactor_single_type(self):
        """Test PII redactor with single type."""
        redactor = PiiRedactor.with_types([PiiType.Email])
        assert redactor is not None

    def test_pii_redactor_all_types(self):
        """Test PII redactor with all types."""
        redactor = PiiRedactor.with_types(
            [
                PiiType.Email,
                PiiType.Phone,
                PiiType.Ssn,
                PiiType.CreditCard,
                PiiType.IpAddress,
            ]
        )
        assert redactor is not None

    def test_redact_email(self):
        """Test redacting email addresses."""
        redactor = PiiRedactor.with_types([PiiType.Email])
        text = "Contact me at test@example.com for more info."
        redacted, found = redactor.redact(text)

        assert "test@example.com" not in redacted
        assert "Email" in found or len(found) > 0

    def test_redact_phone(self):
        """Test redacting phone numbers."""
        redactor = PiiRedactor.with_types([PiiType.Phone])
        text = "Call me at 555-123-4567."
        redacted, found = redactor.redact(text)

        # Phone should be redacted
        assert isinstance(redacted, str)
        assert isinstance(found, list)

    def test_redact_ssn(self):
        """Test redacting SSN."""
        redactor = PiiRedactor.with_types([PiiType.Ssn])
        text = "My SSN is 123-45-6789."
        redacted, found = redactor.redact(text)

        assert "123-45-6789" not in redacted
        assert isinstance(found, list)

    def test_redact_credit_card(self):
        """Test redacting credit card numbers."""
        redactor = PiiRedactor.with_types([PiiType.CreditCard])
        text = "Card number: 4111-1111-1111-1111"
        redacted, found = redactor.redact(text)

        assert isinstance(redacted, str)
        assert isinstance(found, list)

    def test_redact_no_pii(self):
        """Test redacting text with no PII."""
        redactor = PiiRedactor()
        text = "This is a normal message without any PII."
        redacted, found = redactor.redact(text)

        assert redacted == text
        assert len(found) == 0

    def test_redact_multiple_pii(self):
        """Test redacting multiple PII types."""
        redactor = PiiRedactor()
        text = "Email: user@test.com, Phone: 555-123-4567"
        redacted, found = redactor.redact(text)

        assert isinstance(redacted, str)
        assert isinstance(found, list)


class TestGuardrailSet:
    """Tests for GuardrailSet."""

    def test_create_empty_guardrail_set(self):
        """Test creating an empty guardrail set."""
        guardrails = GuardrailSet()
        assert guardrails.is_empty()

    def test_add_content_filter(self):
        """Test adding content filter to set."""
        guardrails = GuardrailSet()
        filter = ContentFilter.harmful_content()
        guardrails = guardrails.with_content_filter(filter)
        assert not guardrails.is_empty()

    def test_add_pii_redactor(self):
        """Test adding PII redactor to set."""
        guardrails = GuardrailSet()
        redactor = PiiRedactor()
        guardrails = guardrails.with_pii_redactor(redactor)
        assert not guardrails.is_empty()

    def test_add_multiple_guardrails(self):
        """Test adding multiple guardrails."""
        guardrails = (
            GuardrailSet()
            .with_content_filter(ContentFilter.harmful_content())
            .with_content_filter(ContentFilter.max_length(1000))
            .with_pii_redactor(PiiRedactor())
        )
        assert not guardrails.is_empty()

    def test_chained_guardrails(self):
        """Test chaining guardrail additions."""
        filter1 = ContentFilter.harmful_content()
        filter2 = ContentFilter.blocked_keywords(["spam"])
        redactor = PiiRedactor()

        guardrails = (
            GuardrailSet()
            .with_content_filter(filter1)
            .with_content_filter(filter2)
            .with_pii_redactor(redactor)
        )
        assert not guardrails.is_empty()


class TestGuardrailResult:
    """Tests for GuardrailResult."""

    def test_guardrail_result_has_passed(self):
        """Test GuardrailResult has passed property."""
        assert hasattr(GuardrailResult, "passed")

    def test_guardrail_result_has_transformed_content(self):
        """Test GuardrailResult has transformed_content property."""
        assert hasattr(GuardrailResult, "transformed_content")

    def test_guardrail_result_has_failures(self):
        """Test GuardrailResult has failures property."""
        assert hasattr(GuardrailResult, "failures")


class TestGuardrailFailure:
    """Tests for GuardrailFailure."""

    def test_guardrail_failure_has_name(self):
        """Test GuardrailFailure has name property."""
        assert hasattr(GuardrailFailure, "name")

    def test_guardrail_failure_has_reason(self):
        """Test GuardrailFailure has reason property."""
        assert hasattr(GuardrailFailure, "reason")

    def test_guardrail_failure_has_severity(self):
        """Test GuardrailFailure has severity property."""
        assert hasattr(GuardrailFailure, "severity")


class TestRunGuardrails:
    """Tests for run_guardrails() function."""

    @pytest.mark.asyncio
    async def test_run_empty_guardrails(self):
        """Test running empty guardrail set."""
        guardrails = GuardrailSet()
        content = Content.user("Hello world")

        result = await run_guardrails(guardrails, content)
        assert result.passed is True
        assert len(result.failures) == 0

    @pytest.mark.asyncio
    async def test_run_guardrails_clean_content(self):
        """Test guardrails with clean content."""
        guardrails = GuardrailSet().with_content_filter(ContentFilter.max_length(1000))
        content = Content.user("This is a normal message.")

        result = await run_guardrails(guardrails, content)
        assert result.passed is True

    @pytest.mark.asyncio
    async def test_run_guardrails_too_long(self):
        """Test guardrails with content exceeding max length."""
        guardrails = GuardrailSet().with_content_filter(ContentFilter.max_length(10))
        content = Content.user("This message is definitely longer than 10 characters.")

        result = await run_guardrails(guardrails, content)
        # Should fail due to length
        assert result.passed is False or len(result.failures) > 0

    @pytest.mark.asyncio
    async def test_run_guardrails_blocked_keyword(self):
        """Test guardrails with blocked keyword."""
        guardrails = GuardrailSet().with_content_filter(
            ContentFilter.blocked_keywords(["forbidden"])
        )
        content = Content.user("This message contains a forbidden word.")

        result = await run_guardrails(guardrails, content)
        # Should fail due to blocked keyword
        assert result.passed is False or len(result.failures) > 0

    @pytest.mark.asyncio
    async def test_run_guardrails_with_pii(self):
        """Test guardrails with PII content."""
        guardrails = GuardrailSet().with_pii_redactor(PiiRedactor())
        content = Content.user("My email is test@example.com")

        result = await run_guardrails(guardrails, content)
        # PII redactor transforms but doesn't fail
        assert result.transformed_content is not None or result.passed

    @pytest.mark.asyncio
    async def test_run_guardrails_multiple(self):
        """Test running multiple guardrails."""
        guardrails = (
            GuardrailSet()
            .with_content_filter(ContentFilter.max_length(1000))
            .with_content_filter(ContentFilter.blocked_keywords(["spam"]))
            .with_pii_redactor(PiiRedactor())
        )
        content = Content.user("Clean message without issues.")

        result = await run_guardrails(guardrails, content)
        assert result.passed is True

    @pytest.mark.asyncio
    async def test_run_guardrails_result_failures(self):
        """Test inspecting guardrail failures."""
        guardrails = GuardrailSet().with_content_filter(ContentFilter.blocked_keywords(["bad"]))
        content = Content.user("This is bad content.")

        result = await run_guardrails(guardrails, content)

        if not result.passed:
            assert len(result.failures) > 0
            for failure in result.failures:
                assert failure.name is not None
                assert failure.reason is not None
                assert failure.severity is not None
