import uuid
from datetime import datetime
from email.policy import default
from operator import index

from sqlalchemy import Column, String, Text, DateTime, Enum
import enum
from app.core.database import Base

class TaskStatus(str, enum.Enum):
    BACKLOG = "积压任务"
    READY = "准备阶段"
    IN_PROGRESS = "进行中"
    IN_REVIEW = "审查中"
    DONE = "已完成"

class Task(Base):
    __tablename__ = "任务表"

    id = Column(String, primary_key=True, default=lambda: str(uuid.uuid4()))
    title = Column(String(255), nullable=False)
    description = Column(Text, nullable=True)
    status = Column(Enum(TaskStatus), default=TaskStatus.BACKLOG)
    org_id = Column(String, nullable=False, index=True)
    created_by = Column(String, nullable=False)
    created_at = Column(DateTime, default=datetime.utcnow)
    updated_at = Column(DateTime, default=datetime.utcnow)